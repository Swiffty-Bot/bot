use serde::{Deserialize, Serialize};
use serenity::prelude::*;

#[derive(Serialize, Deserialize, Default)]
pub struct BotLoginInfo {
    pub token: String,
    pub intents: GatewayIntents,
}

pub async fn start_bot(login: BotLoginInfo) {
    todo!();
}