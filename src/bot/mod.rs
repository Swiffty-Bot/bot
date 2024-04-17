use serde::{Deserialize, Serialize};
use serenity::{prelude::*, model::prelude::*};
use tracing::*;
use crate::prelude::*;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref CLIENT: Option<Arc<Mutex<Client>>> = None;
}

#[derive(Serialize, Deserialize, Default)]
pub struct BotLoginInfo {
    pub token: String,
    pub intents: GatewayIntents,
}

pub struct EventsHandler;

#[async_trait]
impl EventHandler for EventsHandler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let name = ctx.cache.current_user().await.name;
        info!("Logged in as {name}");

        lua::send_event("ready", name).await;
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        lua::send_event("message", new_message).await;
    }

    async fn message_update(&self, ctx: Context, old_if_available: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        lua::send_event("message_update", (old_if_available, new, event)).await;
    }

    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, deleted_message_id: MessageId, guild_id: Option<GuildId>) {
        lua::send_event("message_delete", (channel_id, deleted_message_id, guild_id)).await;
    }

    async fn message_delete_bulk(&self, ctx: Context, channel_id: ChannelId, multiple_deleted_messages_ids: Vec<MessageId>, guild_id: Option<GuildId>) {
        lua::send_event("message_delete_bulk", (channel_id, multiple_deleted_messages_ids, guild_id)).await;
    }    
}

pub async fn start_bot(login: BotLoginInfo) {
    let client = Client::builder(login.token, login.intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    let h = tokio::spawn(async {
        if let Err(why) = client.clone().start().await {
            error!("Client error: {:?}", why);
        }
    });

    let client = Arc::new(Mutex::new(client));
    CLIENT = Some(client.clone());

    h.await.unwrap();
}