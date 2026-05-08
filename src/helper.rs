use serenity::all::ChannelId;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::fetch_user::*;

pub async fn send_embed(
    ctx: &Context,
    title: &str,
    desc: &str,
    content: &str,
    msg: &Message,
) -> Result<(), serenity::Error> {
    let embed = CreateEmbed::new()
        .title(title)
        .description(desc)
        .color(0x00ff00);

    let builder = CreateMessage::new().content(content).embed(embed);

    msg.channel_id.send_message(&ctx.http, builder).await?;
    Ok(())
}

pub async fn send_msg(ctx: &Context, channel_id: u64, content: &str) {
    let channel = ChannelId::new(channel_id);

    let _ = channel.say(&ctx.http, content).await;
}

pub async fn build_user_desc(
    client: &reqwest::Client,
    id: &str,
) -> String {
    let bio = fetch_user_bio(client, id).await.unwrap_or("No bio".to_string());
    let status = fetch_user_status(client, id).await.unwrap_or("ERROR 404".to_string());
    let creation = fetch_user_creation(client, id).await.unwrap_or("ERROR 404".to_string());
    let friends = fetch_user_friends(client, id).await.unwrap_or(0);
    let followers = fetch_user_followers(client, id).await.unwrap_or(0);
    let following = fetch_user_following(client, id).await.unwrap_or(0);
    let visits = fetch_user_visits(client, id).await.unwrap_or(0);

    format!(
        "{}\n\n\
        **Info**\n\
        id: `{}`\n\
        Status: `{}`\n\
        Creation Date:\n`{}`\n\n\
        **Statistics**\n\
        Friends: `{}`\n\
        Followers: `{}`\n\
        Following: `{}`\n\n\
        Visits: `{}`",
        bio,id,status,creation,friends,
        followers,following,visits,
    )
}