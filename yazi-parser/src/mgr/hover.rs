use mlua::{ExternalError, IntoLua, Lua, Value};
use yazi_shared::{event::CmdCow, url::Url};

#[derive(Debug, Default)]
pub struct HoverOpt {
	pub url: Option<Url>,
}

impl From<CmdCow> for HoverOpt {
	fn from(mut c: CmdCow) -> Self { Self { url: c.take_first_url() } }
}

impl From<Option<Url>> for HoverOpt {
	fn from(url: Option<Url>) -> Self { Self { url } }
}

impl IntoLua for &HoverOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}

// --- Do
#[derive(Debug)]
pub struct HoverDoOpt {
	pub url: Url,
}

impl From<Url> for HoverDoOpt {
	fn from(url: Url) -> Self { Self { url } }
}

impl IntoLua for &HoverDoOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
