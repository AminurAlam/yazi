use yazi_shared::{event::CmdCow, url::Url};

pub struct ToggleAllOpt {
	pub urls:  Vec<Url>,
	pub state: Option<bool>,
}

impl From<CmdCow> for ToggleAllOpt {
	fn from(mut c: CmdCow) -> Self {
		let mut urls = Vec::with_capacity(c.len());
		for i in 0..c.len() {
			match c.take_url(i) {
				Some(url) => urls.push(url),
				None => break,
			}
		}

		Self {
			urls,
			state: match c.str("state") {
				Some("on") => Some(true),
				Some("off") => Some(false),
				_ => None,
			},
		}
	}
}

impl From<Option<bool>> for ToggleAllOpt {
	fn from(state: Option<bool>) -> Self { Self { urls: vec![], state } }
}
