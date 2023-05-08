use mlua::prelude::*;
use regex::Regex;

mod captures;

pub use captures::LuaCaptures;

use self::captures::match_to_lua_table;

#[derive(Debug, Clone)]
pub struct LuaRegex {
    inner_regex: Regex,
}

impl LuaRegex {
    pub fn new(regex: &str) -> LuaResult<Self> {
        let inner_regex = Regex::new(regex).to_lua_err()?;
        Ok(Self { inner_regex })
    }
}

impl LuaUserData for LuaRegex {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("IsMatch", |lua, this, text: String| {
            this.inner_regex.is_match(&text).to_lua(lua)
        });

        methods.add_method("Find", |lua, this, text: String| {
            if let Some(mat) = this.inner_regex.find(&text) {
                Ok(LuaValue::Table(match_to_lua_table(lua, mat)?))
            } else {
                Ok(LuaNil)
            }
        });

        methods.add_method("FindAll", |lua, this, text: String| {
            this.inner_regex
                .find_iter(&text)
                .map(|mat| match_to_lua_table(lua, mat))
                .collect::<LuaResult<Vec<_>>>()
                .and_then(|matches| matches.to_lua(lua))
        });

        methods.add_method("Captures", |lua, this, text: String| {
            if let Some(caps) = this.inner_regex.captures(&text) {
                let lua_caps = LuaCaptures::new(caps);
                Ok(lua_caps.to_lua(lua)?)
            } else {
                Ok(LuaNil)
            }
        });

        methods.add_method(
            "CapturesAll",
            |_lua, _this, _text: String| -> LuaResult<()> { todo!() },
        );

        methods.add_method("Split", |_lua, _this, _text: String| -> LuaResult<()> {
            todo!()
        });
    }
}
