use std::fs::Metadata;

use bitflags::bitflags;
use yazi_shared::url::Url;

bitflags! {
	#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
	pub struct ChaKind: u8 {
		const DIR    = 0b00000001;

		const HIDDEN = 0b00000010;
		const LINK   = 0b00000100;
		const ORPHAN = 0b00001000;

		const DUMMY  = 0b00010000;
		#[cfg(windows)]
		const SYSTEM = 0b00100000;
	}
}

impl ChaKind {
	#[inline]
	pub(super) fn hidden(_url: &Url, _meta: &Metadata) -> Self {
		let mut me = Self::empty();

		#[cfg(unix)]
		if _url.urn().is_hidden() {
			me |= Self::HIDDEN;
		}
		#[cfg(windows)]
		{
			use std::os::windows::fs::MetadataExt;

			use windows_sys::Win32::Storage::FileSystem::{FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_SYSTEM};
			if _meta.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0 {
				me |= Self::HIDDEN;
			}
			if _meta.file_attributes() & FILE_ATTRIBUTE_SYSTEM != 0 {
				me |= Self::SYSTEM;
			}
		}

		me
	}
}
