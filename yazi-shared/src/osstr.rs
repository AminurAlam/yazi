use std::{borrow::Cow, ffi::{OsStr, OsString}};

pub trait IntoOsStr<'a> {
	type Error;

	fn into_os_str(self) -> Result<Cow<'a, OsStr>, Self::Error>;
}

impl<'a> IntoOsStr<'a> for Cow<'a, [u8]> {
	type Error = anyhow::Error;

	fn into_os_str(self) -> Result<Cow<'a, OsStr>, Self::Error> {
		#[cfg(unix)]
		{
			use std::os::unix::ffi::{OsStrExt, OsStringExt};
			Ok(match self {
				Cow::Borrowed(b) => Cow::Borrowed(OsStr::from_bytes(b)),
				Cow::Owned(b) => Cow::Owned(OsString::from_vec(b)),
			})
		}
		#[cfg(windows)]
		{
			Ok(match self {
				Cow::Borrowed(b) => Cow::Borrowed(OsStr::new(str::from_utf8(b)?)),
				Cow::Owned(b) => Cow::Owned(OsString::from(String::from_utf8(b)?)),
			})
		}
	}
}

impl<'a> IntoOsStr<'a> for &'a [u8] {
	type Error = anyhow::Error;

	fn into_os_str(self) -> Result<Cow<'a, OsStr>, Self::Error> { Cow::Borrowed(self).into_os_str() }
}

// --- OsStrJoin
pub trait OsStrJoin {
	fn join(&self, sep: &OsStr) -> OsString;
}

impl<T> OsStrJoin for Vec<T>
where
	T: AsRef<OsStr>,
{
	fn join(&self, sep: &OsStr) -> OsString {
		if self.is_empty() {
			return OsString::new();
		}

		let mut result = OsString::new();
		for (i, item) in self.iter().enumerate() {
			if i > 0 {
				result.push(sep);
			}
			result.push(item.as_ref());
		}
		result
	}
}
