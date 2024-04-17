use tracing::*;
use tracing_subscriber::EnvFilter;

use swiffty_bot::prelude::*;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env()
            .unwrap_or(EnvFilter::INFO))
        .init();

    info!("Loading bot");
    let login_info = bot::BotLoginInfo {
        token: std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment"),
        intents: serenity::GatewayIntents::all(),
    };
    
    bot::start_bot(login_info).await;
}