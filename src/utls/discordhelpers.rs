use std::str;

use serenity::{
    builder::{CreateEmbed, CreateMessage},
    http::Http,
    model::prelude::*,
};
use serenity::all::CreateEmbedFooter;

use crate::utls::constants::*;
use crate::parser;

pub async fn dispatch_embed(
    http: &Http,
    channel: ChannelId,
    emb: CreateEmbed,
) -> serenity::Result<Message> {
    let emb_msg = embed_message(emb);
    channel.send_message(&http, emb_msg).await
}

pub fn embed_message(emb: CreateEmbed) -> CreateMessage {
    CreateMessage::default().embed(emb)
}

pub fn build_fail_embed(author: &User, err: &str) -> CreateEmbed {
    let footer = CreateEmbedFooter::new(format!("Requested by: {}", author.name));

    CreateEmbed::new()
        .color(COLOR_FAIL)
        .title("Critical error:")
        .description(err)
        .thumbnail(ICON_FAIL)
        .footer(footer)
}

pub fn build_amx_embed(data : parser::Item) -> CreateEmbed {
    CreateEmbed::default()
        .color(COLOR_AMX)
        .title("New AMX Plugin Posted:")
        .thumbnail(ICON_NOTIFY)
        .field("Title", data.title, false)
        .field("Author", data.author, false)
        .field("Link", format!("[Click Here]({})", data.link), false)
}

pub fn build_sm_embed(data : parser::Item) -> CreateEmbed {
    CreateEmbed::default()
        .color(COLOR_SM)
        .title("New SM Plugin Posted:")
        .thumbnail(ICON_NOTIFY)
        .field("Title", data.title, false)
        .field("Author", data.author, false)
        .field("Link", format!("[Click Here]({})", data.link), false)
}