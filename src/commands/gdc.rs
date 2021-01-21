use std::collections::HashMap;
use std::fs::OpenOptions;

use serenity::framework::standard::{macros::command, Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::builder::CreateEmbed;

use gdcrunner::downloader::GDCError;
use gdcrunner::GDCManager;

use crate::utls::discordhelpers;
use crate::cache::BotInfo;
use crate::utls::constants::{ICON_GDC, COLOR_GDC};

#[command]
#[owners_only]
pub async fn gdc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if args.is_empty() {
        return Err(CommandError::from(
            "Please supply a game to execute gamedata checker on.\n\nExample: -gdc csgo",
        ));
    }
    let app = args.parse::<String>().unwrap();

    let appid = gdcrunner::appid_translator::get_appid(&app);
    if appid == 0 {
        return Err(CommandError::from(
            "Invalid or unsupported target.",
        ));
    }


    let data = ctx.data.read().await;
    let info = data.get::<BotInfo>().unwrap().read().await;

    let sourcemod_dir = info.get("SOURCEMOD_DIR").unwrap();
    let downloads_dir = info.get("DOWNLOADS_DIR").unwrap();
    let depot_dir = info.get("DEPOT_DIR").unwrap();

    let mut emb = CreateEmbed::default();
    emb.title("Gamedata Checker");
    emb.thumbnail(ICON_GDC);
    emb.color(COLOR_GDC);
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
        f
    }).await?;

    Ok(())
}

async fn build_results_embed(msg : &mut Message, http : &Context, results : &HashMap<String, std::result::Result<bool, GDCError>>) {
    let tag = msg.author.tag();
    let text = msg.content.clone();
    msg.edit(http, |f| {
        f.embed(|e| {
            for (k, v) in results {
                if v.is_err() {
                    e.field(k, format!("```\n{}\n```", v.as_ref().err().unwrap()), false);
                }
            }

            e.title("Gamedata Checker");
            e.color(COLOR_GDC);
            e.description(format!("{}\nExecution completed.", text));
            e.thumbnail(ICON_GDC);
            e.footer(|f| f.text(format!("Requested by: {}", tag)));
            e
        })
    }).await.expect("Unable to edit message.")
}

async fn update_msg(msg : &mut Message, http : &Context, text : &str) {
    let old_text = msg.content.clone();

    let tag = msg.author.tag();
    msg.edit(http, |f| f.embed(|e| {
        e.title("Gamedata Checker");
        e.description(format!("{}\n{}", old_text, text));
        e.thumbnail(ICON_GDC);
        e.color(COLOR_GDC);
        e.footer(|f| f.text(format!("Requested by: {}", tag)));
        e
    })).await.expect("Unable to edit message.")
}
