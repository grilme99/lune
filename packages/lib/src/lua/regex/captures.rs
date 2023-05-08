use mlua::prelude::*;
use regex::{Captures, Match};

pub fn match_to_lua_table<'a>(lua: &'a Lua, mat: Match<'_>) -> LuaResult<LuaTable<'a>> {
    // Cannot use TableBuilder because of lifetime issues. It requires Lua live for 'static, but we need to
    // tie the lua lifetime with LuaTable (which cannot be 'static).
    let table = lua.create_table()?;

    table.set("Start", mat.start())?;
    table.set("End", mat.end())?;
    table.set("Empty", mat.is_empty())?;
    table.set("Length", mat.len())?;
    table.set("Text", mat.as_str())?;

    table.set_readonly(true);

    Ok(table)
}

#[derive(Debug)]
pub struct LuaMatch<'a> {
    inner_match: Match<'a>,
}

impl<'a> LuaMatch<'a> {
    pub fn new(inner_match: Match<'a>) -> Self {
        Self { inner_match }
    }
}

impl LuaUserData for LuaMatch<'_> {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Start", |_, this| Ok(this.inner_match.start()));
        fields.add_field_method_get("End", |_, this| Ok(this.inner_match.end()));
        fields.add_field_method_get("Empty", |_, this| Ok(this.inner_match.is_empty()));
        fields.add_field_method_get("Length", |_, this| Ok(this.inner_match.len()));
        fields.add_field_method_get("Text", |_, this| Ok(this.inner_match.as_str()));
    }

    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, _: ()| {
            Ok(this.inner_match.as_str())
        });

        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: LuaValue| match other {
            LuaValue::String(other) => Ok(this.inner_match.as_str() == other.to_str()?),
            LuaValue::UserData(other) => {
                if let Ok(other) = other.borrow::<LuaMatch>() {
                    Ok(this.inner_match == other.inner_match)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        });
    }
}

#[derive(Debug)]
pub struct LuaCaptures<'a> {
    inner_captures: Captures<'a>,
}

impl<'a> LuaCaptures<'a> {
    pub fn new(captures: Captures<'a>) -> Self {
        Self {
            inner_captures: captures,
        }
    }
}

impl LuaUserData for LuaCaptures<'_> {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_function(LuaMetaMethod::ToString, |_, _: ()| Ok("Captures"));

        methods.add_meta_method(LuaMetaMethod::Len, |_, this, _: ()| {
            Ok(this.inner_captures.len())
        });

        methods.add_meta_method(
            LuaMetaMethod::Index,
            |_lua, _this, _index: LuaValue| -> LuaResult<LuaValue> {
                todo!()
                //
            },
        );

        methods.add_method("Get", |lua, this, index: usize| {
            // Lua indexes start at 1, decrease the index so that it works with Rust Regex
            let index = index - 1;

            if let Some(mat) = this.inner_captures.get(index) {
                Ok(LuaValue::Table(match_to_lua_table(lua, mat)?))
            } else {
                Ok(LuaNil)
            }
        });

        methods.add_method("Name", |lua, this, name: String| {
            if let Some(mat) = this.inner_captures.name(&name) {
                Ok(LuaValue::Table(match_to_lua_table(lua, mat)?))
            } else {
                Ok(LuaNil)
            }
        });
    }
}
