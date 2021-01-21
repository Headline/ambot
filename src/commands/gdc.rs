use std::collections::HashMap;
use std::fs::OpenOptions;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::builder::CreateEmbed;

use gdcrunner::downloader::GDCError;
use gdcrunner::GDCManager;

use crate::utls::discordhelpers;
use crate::cache::BotInfo;

#[command]
#[owners_only]
pub async fn gdc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let appid = args.parse::<i32>().unwrap();

    let data = ctx.data.read().await;
    let info = data.get::<BotInfo>().unwrap().read().await;

    let sourcemod_dir = info.get("SOURCEMOD_DIR").unwrap();
    let downloads_dir = info.get("DOWNLOADS_DIR").unwrap();
    let depot_dir = info.get("DEPOT_DIR").unwrap();

    let mut emb = CreateEmbed::default();
    emb.description(format!("Downloading app id '{}'", appid));
    emb.footer(|f| f.text(format!("Requested by: {}", &msg.author.tag())));

    let mut emb_msg = discordhelpers::embed_message(emb);
    let mut message = msg.channel_id
        .send_message(&ctx.http, |_| &mut emb_msg)
        .await?;

    let gdc = GDCManager::new(appid, sourcemod_dir, downloads_dir, depot_dir);

    match gdc.download_game().await {
        Ok(t) => {
            if !t.success() {
                update_msg(& mut message, &ctx, &format!("Exited with status code {}", t)).await
            }
        }
        Err(e) => {
            update_msg(& mut message, &ctx, &format!("Fatal error: {}", e)).await
        }
    }

    update_msg(& mut message, &ctx, "Download completed. Running gdc...").await;
    let mut file = OpenOptions::new()
        .write(true) // <--------- this
        .create(true)
        .open("output.log")
        .unwrap();
    let results = gdc.check_gamedata(& mut file).await;

    build_results_embed(&mut message, ctx, &results).await;
    msg.channel_id.send_files(&ctx.http, vec!["output.log"], |f| {
        f.content("GDC execution complete.");
        f
    }).await?;

    Ok(())
}

async fn build_results_embed(msg : &mut Message, http : &Context, results : &HashMap<String, std::result::Result<bool, GDCError>>) {
    let tag = msg.author.tag();
    msg.edit(http, |f| {
        f.embed(|e| {
            for (k, v) in results {
                if v.is_err() {
                    e.field(k, format!("```\n{}\n```", v.as_ref().err().unwrap()), false);
                }
            }

            e.description("Execution completed.");
            e.footer(|f| f.text(format!("Requested by: {}", tag)));
            e
        })
    }).await.expect("Unable to edit message.")
}

async fn update_msg(msg : &mut Message, http : &Context, text : &str) {
    let tag = msg.author.tag();
    msg.edit(http, |f| f.embed(|e| {
        e.description(text);
        e.footer(|f| f.text(format!("Requested by: {}", tag)));
        e
    })).await.expect("Unable to edit message.")
}
