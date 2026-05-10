use std::io::{self, Write};

mod fetch_user;
mod helper;

use fetch_user::*;
use helper::*;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::sync::Arc;

fn request_token() -> String {
    let mut token = String::new();

    print!("Input Bot Token > ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut token).expect("Failed to read.");

    token.trim().to_string()
}

#[tokio::main]
async fn main() {
    init().await;
}

pub async fn init() {
    struct Handler {
        client: Arc<reqwest::Client>,
    }

    #[async_trait]
    impl EventHandler for Handler {
        async fn ready(&self, _ctx: Context, _ready: Ready) {
            //..
        }

        async fn message(&self, ctx: Context, msg: Message) {
            if msg.author.bot {
                return;
            }

            let mut parts = msg.content.split_whitespace();
            let Some(command) = parts.next() else {
                return;
            };

            match command {
                "+help" => {
                    let text = "\
            **Commands**\n\
            `fetch id <id>` / `whois id <id>`\n\
            `fetch name <name>` / `whois name <name>`\n\
            `fetch newest` / `fetch latest`\n\
            `whois newest` / `whois latest`";
                    let _ = msg.channel_id.say(&ctx.http, text).await;
                }

                "fetch" | "whois" => {
                    let Some(kind) = parts.next() else {
                        return;
                    };

                    match kind {
                        "id" => {
                            let Some(arg) = parts.next() else {
                                return;
                            };

                            if matches!(arg, "latest" | "newest") {
                                let _ = msg.channel_id.say(
                        &ctx.http,
                        "Attempting to fetch latest user, please wait..\n-# Fetching can take up to 15 seconds *(Binary Search Algorithm)*",
                    ).await;

                                let id = fetch_newest_user(&self.client)
                                    .await
                                    .unwrap_or_else(|| "0".to_string());

                                let username = fetch_user_name(&self.client, &id)
                                    .await
                                    .unwrap_or_else(|| "None".to_string());

                                let usr = format!("{username} (NEWEST)");
                                let desc = build_user_desc(&self.client, &id).await;
                                let _ = send_embed(&ctx, &usr, &desc, "", &msg).await;
                                return;
                            }

                            match fetch_user::fetch_user_name(&self.client, arg).await {
                                Some(username) => {
                                    let desc = build_user_desc(&self.client, arg).await;
                                    let _ = send_embed(&ctx, &username, &desc, "", &msg).await;
                                }
                                None => {
                                    let _ = msg.channel_id.say(&ctx.http, "User not found").await;
                                }
                            }
                        }

                        "name" => {
                            let Some(arg) = parts.next() else {
                                return;
                            };

                            let id = fetch_id_by_name(&self.client, arg).await;

                            match id.as_deref() {
                                Some(id) => match fetch_user::fetch_user_name(&self.client, id)
                                    .await
                                {
                                    Some(username) => {
                                        let desc = build_user_desc(&self.client, id).await;
                                        let _ = send_embed(&ctx, &username, &desc, "", &msg).await;
                                    }
                                    None => {
                                        let _ =
                                            msg.channel_id.say(&ctx.http, "User not found").await;
                                    }
                                },
                                None => {
                                    let _ = msg.channel_id.say(&ctx.http, "User not found").await;
                                }
                            }
                        }

                        "latest" | "newest" => {
                            let _ = msg.channel_id.say(
                    &ctx.http,
                    "Attempting to fetch latest user, please wait..\n-# Fetching can take up to 15 seconds *(Binary Search Algorithm)*",
                ).await;

                            let id = fetch_newest_user(&self.client)
                                .await
                                .unwrap_or_else(|| "0".to_string());

                            let username = fetch_user_name(&self.client, &id)
                                .await
                                .unwrap_or_else(|| "None".to_string());

                            let usr = format!("{username} (NEWEST)");
                            let desc = build_user_desc(&self.client, &id).await;
                            let _ = send_embed(&ctx, &usr, &desc, "", &msg).await;
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }

    let token = request_token();
    println!("\nBot initialized");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        client: Arc::new(reqwest::Client::new()),
    };

    let mut client = serenity::Client::builder(token, intents)
        .event_handler(handler)
        .await
        .unwrap();

    client.start().await.unwrap();
}
