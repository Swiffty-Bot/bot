pub mod plugins;

use mlua::prelude::*;
use lazy_static::lazy_static;
use tracing::*;
use std::sync::Arc;

lazy_static! {
    pub static ref LUA: Arc<Lua> = {
        info!("Loading Lua interpreter");
        Arc::new(load_lua())
    };
}

const MAIN: &str = include_str!("main.luau");
const EVENTS: &str = include_str!("events.luau");

pub fn load_lua() -> Lua {
    let lua = Lua::new();

    lua.load(MAIN).exec().unwrap();
    lua.load(EVENTS).exec().unwrap();

    plugins::load_plugins(&lua);

    lua
}

pub async fn send_event<V: IntoLuaMulti>(event_name: &str, event_data: V) -> Result<(), LuaError> {
    let lua = LUA.clone();
    
    lua.globals().call_async_function("Swiffty:triggerEvent", (event_name, event_data)).await?;

    Ok(())
}