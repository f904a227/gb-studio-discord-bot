use serenity::{async_trait, model::gateway::Ready, prelude::*};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.tag());

        println!("Ready!");
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_BOT_TOKEN")
        .expect("Failed to fetch the environment variable DISCORD_BOT_TOKEN");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Failed to create the client");

    if let Err(err) = client.start().await {
        eprintln!("Client error: {err:?}");
    }
}
