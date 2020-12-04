use std::str;
use std::sync::Arc;

use serenity::{
    builder::{CreateEmbed, CreateMessage},
    http::Http,
    model::prelude::*,
};

use crate::utls::constants::*;
use crate::parser;


pub async fn manual_dispatch(http: Arc<Http>, id: u64, emb: CreateEmbed) {
    match serenity::model::id::ChannelId(id)
        .send_message(&http, |m| {
            m.embed(|mut e| {
                e.0 = emb.0;
                e
            })
        })
        .await
    {
        Ok(m) => m,
        Err(e) => return error!("Unable to dispatch manually: {}", e),
    };
}

pub fn embed_message(emb: CreateEmbed) -> CreateMessage<'static> {
    let mut msg = CreateMessage::default();
    msg.embed(|e| {
        e.0 = emb.0;
        e
    });
    msg
}

pub fn build_fail_embed(author: &User, err: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_FAIL);
    embed.title("Critical error:");
    embed.description(err);
    embed.thumbnail(ICON_FAIL);
    embed.footer(|f| f.text(format!("Requested by: {}", author.tag())));
    embed
}

pub fn build_amx_embed(data : parser::Item) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_AMX);
    embed.title("New AMX Plugin Posted:");
    embed.thumbnail(ICON_NOTIFY);
    embed.field("Title", data.title, false);
    embed.field("Author", data.author, false);
    embed.field("Link", format!("[Click Here]({})", data.link), false);
    embed
}

pub fn build_sm_embed(data : parser::Item) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_SM);
    embed.title("New SM Plugin Posted:");
    embed.thumbnail(ICON_NOTIFY);
    embed.field("Title", data.title, false);
    embed.field("Author", data.author, false);
    embed.field("Link", format!("[Click Here]({})", data.link), false);
    embed
}