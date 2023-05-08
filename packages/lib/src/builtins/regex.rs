use mlua::prelude::*;

use crate::lua::{regex::LuaRegex, table::TableBuilder};

pub fn create(lua: &'static Lua) -> LuaResult<LuaTable> {
    TableBuilder::new(lua)?
        .with_function("new", new_regex)?
        .build_readonly()
}

fn new_regex(lua: &'static Lua, regex_str: String) -> LuaResult<LuaValue> {
    LuaRegex::new(&regex_str)?.to_lua(lua)
}
