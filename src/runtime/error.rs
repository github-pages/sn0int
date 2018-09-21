use engine::ctx::State;
use hlua::{self, AnyLuaValue};
use std::sync::Arc;


pub fn clear_err(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("clear_err", hlua::function0(move || {
        state.clear_error()
    }))
}

pub fn last_err(lua: &mut hlua::Lua, state: Arc<State>) {
    lua.set("last_err", hlua::function0(move || -> AnyLuaValue {
        match state.last_error() {
            Some(err) => AnyLuaValue::LuaString(err),
            None => AnyLuaValue::LuaNil,
        }
    }))
}
