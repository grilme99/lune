use std::{
    fs::read_to_string,
    path::{PathBuf, MAIN_SEPARATOR},
};

use clap::{CommandFactory, Parser};
use mlua::{Lua, MultiValue, Result, ToLua};

use crate::{
    lune::{console::LuneConsole, fs::LuneFs, net::LuneNet, process::LuneProcess},
    utils::GithubClient,
};

/// Lune CLI
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the file to run, or the name
    /// of a luau file in a lune directory
    ///
    /// Can be omitted when downloading type definitions
    script_path: Option<String>,
    /// Arguments to pass to the file as vararg (...)
    script_args: Vec<String>,
    /// Pass this flag to download the Selene type
    /// definitions file to the current directory
    #[clap(long)]
    download_selene_types: bool,
    /// Pass this flag to download the Luau type
    /// definitions file to the current directory
    #[clap(long)]
    download_luau_types: bool,
}

impl Cli {
    #[allow(dead_code)]
    pub fn from_path<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            script_path: Some(path.into()),
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn from_path_with_args<S, A>(path: S, args: A) -> Self
    where
        S: Into<String>,
        A: Into<Vec<String>>,
    {
        Self {
            script_path: Some(path.into()),
            script_args: args.into(),
            ..Default::default()
        }
    }

    pub async fn run(self) -> Result<()> {
        // Download definition files, if wanted
        let download_types_requested = self.download_selene_types || self.download_luau_types;
        if download_types_requested {
            let client = GithubClient::new().map_err(mlua::Error::external)?;
            let release = client
                .fetch_release_for_this_version()
                .await
                .map_err(mlua::Error::external)?;
            if self.download_selene_types {
                println!("Downloading Selene type definitions...");
                client
                    .fetch_release_asset(&release, "lune.yml")
                    .await
                    .map_err(mlua::Error::external)?;
            }
            if self.download_luau_types {
                println!("Downloading Luau type definitions...");
                client
                    .fetch_release_asset(&release, "luneTypes.d.luau")
                    .await
                    .map_err(mlua::Error::external)?;
            }
        }
        if self.script_path.is_none() {
            if !download_types_requested {
                // HACK: We know that we didn't get any arguments here but since
                // script_path is optional clap will not error on its own, to fix
                // we will duplicate the cli command and make arguments required,
                // which will then fail and print out the normal help message
                let cmd = Cli::command();
                cmd.arg_required_else_help(true).get_matches();
            } else {
                // Only downloading types without running a script is completely
                // fine, and we should just exit the program normally afterwards
                return Ok(());
            }
        }
        // Parse and read the wanted file
        let file_path = find_parse_file_path(&self.script_path.unwrap())?;
        let file_contents = read_to_string(file_path)?;
        // Create a new lua state and add in all lune globals
        let lua = Lua::new();
        let globals = lua.globals();
        globals.set("console", LuneConsole::new())?;
        globals.set("fs", LuneFs::new())?;
        globals.set("net", LuneNet::new())?;
        globals.set("process", LuneProcess::new())?;
        lua.sandbox(true)?;
        // Load & call the file with the given args
        let lua_args = self
            .script_args
            .iter()
            .map(|value| value.to_owned().to_lua(&lua))
            .collect::<Result<Vec<_>>>()?;
        lua.load(&file_contents)
            .call_async(MultiValue::from_vec(lua_args))
            .await?;
        Ok(())
    }
}

fn find_luau_file_path(path: &str) -> Option<PathBuf> {
    let file_path = PathBuf::from(path);
    if let Some(ext) = file_path.extension() {
        match ext {
            e if e == "lua" || e == "luau" && file_path.exists() => Some(file_path),
            _ => None,
        }
    } else {
        let file_path_lua = PathBuf::from(path).with_extension("lua");
        if file_path_lua.exists() {
            Some(file_path_lua)
        } else {
            let file_path_luau = PathBuf::from(path).with_extension("luau");
            if file_path_luau.exists() {
                Some(file_path_luau)
            } else {
                None
            }
        }
    }
}

fn find_parse_file_path(path: &str) -> Result<PathBuf> {
    let parsed_file_path = find_luau_file_path(path)
        .or_else(|| find_luau_file_path(&format!("lune{MAIN_SEPARATOR}{path}")))
        .or_else(|| find_luau_file_path(&format!(".lune{MAIN_SEPARATOR}{path}")));
    if let Some(file_path) = parsed_file_path {
        if file_path.exists() {
            Ok(file_path)
        } else {
            Err(mlua::Error::RuntimeError(format!(
                "File does not exist at path: '{}'",
                path
            )))
        }
    } else {
        Err(mlua::Error::RuntimeError(format!(
            "Invalid file path: '{}'",
            path
        )))
    }
}