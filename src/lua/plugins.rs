use mlua::prelude::*;
use tracing::*;
use serde::Deserialize;
use walkdir::WalkDir;

pub fn load_plugins(lua: &Lua) {
    info!("Loading plugins");
    let plugins_dir = std::env::current_exe().unwrap().join("../plugins");

    for entry in WalkDir::new(plugins_dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().is_dir() {
            load_plugin(lua, path.to_path_buf());
        }
    }
}

#[derive(Deserialize)]
pub struct PluginManifest {
    pub name: String,

    #[serde(default = "(This plugin does not have a description)")]
    pub description: String,
    
    pub version: String,
    pub repo: Option<String>,

    #[serde(default = "src")]
    pub src: String,
}

pub fn load_plugin(lua: &Lua, dir: PathBuf) {
    let manifest = dir.join("plugin.toml");

    if !manifest.exists() {
        error!("Plugin at {dir:?} does not have a manifest");
        return;
    }

    let manifest = std::fs::read_to_string(manifest).unwrap();
    let manifest: PluginManifest = toml::from_str(&manifest).unwrap();

    info!("Loading plugin {} v{}", manifest.name, manifest.version);

    let src = dir.join(manifest.src);

    if !src.exists() {
        error!("Plugin at {dir:?} does not have a src file");
        return;
    }

    for entry in WalkDir::new(src) {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().is_file() && path.extension().unwrap().to_str().unwrap().starts_with("lua") {
            let src = std::fs::read_to_string(path).unwrap();
            lua.load(&src).exec().unwrap();
        }
    }
}