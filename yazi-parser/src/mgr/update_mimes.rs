use std::collections::HashMap;

use anyhow::bail;
use mlua::{ExternalError, IntoLua, Lua, Value};
use yazi_shared::event::{CmdCow, Data, DataKey};

#[derive(Debug)]
pub struct UpdateMimesOpt {
	pub updates: HashMap<DataKey, Data>,
}

impl TryFrom<CmdCow> for UpdateMimesOpt {
	type Error = anyhow::Error;

	fn try_from(mut c: CmdCow) -> Result<Self, Self::Error> {
		let Some(updates) = c.try_take("updates").and_then(Data::into_dict) else {
			bail!("Invalid 'updates' argument in UpdateMimesOpt");
		};

		Ok(Self { updates })
	}
}

impl IntoLua for &UpdateMimesOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
