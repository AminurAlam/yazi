use std::{collections::VecDeque, fs::ReadDir, future::poll_fn, io, mem, pin::Pin, task::{Poll, ready}, time::{Duration, Instant}};

use tokio::task::JoinHandle;
use yazi_shared::{Either, url::Url};

use crate::services;

type Task = Either<Url, ReadDir>;

pub enum SizeCalculator {
	Idle((VecDeque<Task>, Option<u64>)),
	Pending(JoinHandle<(VecDeque<Task>, Option<u64>)>),
}

impl SizeCalculator {
	pub async fn new(url: &Url) -> io::Result<Self> {
		let u = url.to_owned();
		tokio::task::spawn_blocking(move || {
			let meta = services::symlink_metadata_sync(&u)?;
			if !meta.is_dir() {
				return Ok(Self::Idle((VecDeque::new(), Some(meta.len()))));
			}

			let mut buf = VecDeque::from([Either::Right(services::read_dir_sync(u)?)]);
			let size = Self::next_chunk(&mut buf);
			Ok(Self::Idle((buf, size)))
		})
		.await?
	}

	pub async fn total(url: &Url) -> io::Result<u64> {
		let mut it = Self::new(url).await?;
		let mut total = 0;
		while let Some(n) = it.next().await? {
			total += n;
		}
		Ok(total)
	}

	pub async fn next(&mut self) -> io::Result<Option<u64>> {
		poll_fn(|cx| {
			loop {
				match self {
					Self::Idle((buf, size)) => {
						if let Some(s) = size.take() {
							return Poll::Ready(Ok(Some(s)));
						} else if buf.is_empty() {
							return Poll::Ready(Ok(None));
						}

						let mut buf = mem::take(buf);
						*self = Self::Pending(tokio::task::spawn_blocking(move || {
							let size = Self::next_chunk(&mut buf);
							(buf, size)
						}));
					}
					Self::Pending(handle) => {
						*self = Self::Idle(ready!(Pin::new(handle).poll(cx))?);
					}
				}
			}
		})
		.await
	}

	fn next_chunk(buf: &mut VecDeque<Either<Url, ReadDir>>) -> Option<u64> {
		let (mut i, mut size, now) = (0, 0, Instant::now());
		macro_rules! pop_and_continue {
			() => {{
				buf.pop_front();
				if buf.is_empty() {
					return Some(size);
				}
				continue;
			}};
		}

		while i < 5000 && now.elapsed() < Duration::from_millis(50) {
			i += 1;
			let front = buf.front_mut()?;

			if let Either::Left(u) = front {
				*front = match services::read_dir_sync(u) {
					Ok(it) => Either::Right(it),
					Err(_) => pop_and_continue!(),
				};
			}

			let Some(next) = front.right_mut()?.next() else {
				pop_and_continue!();
			};

			let Ok(ent) = next else { continue };
			let Ok(ft) = ent.file_type() else { continue };
			if ft.is_dir() {
				buf.push_back(Either::Left(ent.path().into()));
			} else if let Ok(meta) = ent.metadata() {
				size += meta.len();
			}
		}
		Some(size)
	}
}
