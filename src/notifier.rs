use std::sync::{Arc};
use tokio::sync::RwLock;

use serenity::prelude::TypeMap;

use crate::parser;
use std::error::Error;
use crate::cache::BotInfo;
use crate::utls::discordhelpers::*;
use serenity::http::Http;
extern crate serde;
extern crate quick_xml;

pub fn start_listening(data: Arc<RwLock<TypeMap>>, http : Arc<Http>) {
    tokio::spawn(async move {
        let http = http.clone();

        let mut amx_cache : Vec<parser::Item> = Vec::new();
        let mut sm_cache : Vec<parser::Item> = Vec::new();
        loop {
            let data = data.read().await;
            let bot_info = data.get::<BotInfo>().unwrap().read().await;
            let channel = bot_info.get("PLUGIN_CHANNEL").unwrap();

            // AMXModX
            let xml = parse_xml("https://forums.alliedmods.net/external.php?newpost=true&forumids=26").await;
            if let Ok(data) = xml {
                // load cache on startup
                if amx_cache.is_empty() {
                    amx_cache = data.channel.items;
                } else {
                    notify_on_new(& mut amx_cache, &data.channel.items, channel, true, http.clone()).await
                }
            }

            // SourceMod
            let xml = parse_xml("https://forums.alliedmods.net/external.php?newpost=true&forumids=108").await;
            if let Ok(data) = xml {
                // load cache on startup
                if sm_cache.is_empty() {
                    sm_cache = data.channel.items;
                } else {
                    notify_on_new(& mut sm_cache, &data.channel.items, channel, false, http.clone()).await;
                }
            }

            tokio::time::sleep(core::time::Duration::new(120, 0)).await;
        }
    });
}


async fn notify_on_new(cache : & mut Vec<parser::Item>, new : &Vec<parser::Item>, channel : &str, amx : bool, http : Arc<Http>) {
    let new_entries : Vec<parser::Item> = new.iter().filter(|&x| !cache.contains(x)).cloned().collect();
    if new_entries.len() == 0 {
       return;
    }

    for x in new_entries {
        cache.push(x.clone());

        let emb;
        if amx {
            emb = build_amx_embed(x);
        }
        else {
            emb = build_sm_embed(x);
        }
        let _ = manual_dispatch(http.clone(), channel.parse::<u64>().unwrap(), emb).await;
    }
}

async fn parse_xml(uri : &str) -> Result<parser::RSS, Box<dyn Error+Sync+Send>>{
    let resp = reqwest::get(uri).await?;

    let mut text = resp.text().await?;

    text = text.replace('&', "&amp;");
    return match quick_xml::de::from_str(&text) {
        Err(e) => Err(e.into()),
        Ok(text) => Ok(text)
    };
}