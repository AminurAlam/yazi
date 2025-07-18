use yazi_shared::{SStr, event::CmdCow};

pub struct SwipeOpt {
	pub step: SStr,
}

impl From<CmdCow> for SwipeOpt {
	fn from(mut c: CmdCow) -> Self { Self { step: c.take_first_str().unwrap_or_default() } }
}
