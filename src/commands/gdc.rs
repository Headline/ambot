use std::collections::HashMap;
use std::fs::OpenOptions;

use serenity::framework::standard::{macros::command, Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::builder::CreateEmbed;

use gdcrunner::gameinfo::*;
use gdcrunner::downloader::GDCError;
use gdcrunner::GDCManager;

use crate::utls::discordhelpers;
use crate::cache::BotInfo;
use crate::utls::constants::{ICON_GDC, COLOR_GDC};
use std::process::{Command, Stdio};

#[command]
#[owners_only]
pub async fn gdc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    if args.is_empty() {
        return Err(CommandError::from(
            "Please supply a game to execute gamedata checker on.\n\nExample: -gdc csgo",
        ));
    }
    let app = args.parse::<String>().unwrap();

    let cache = GameCache::new();
    let game_option = cache.lookup_shortname(&app);
    if game_option.is_none() {
        return Err(CommandError::from(
            "Invalid or unsupported target.",
        ));
    }
    let game = game_option.unwrap().clone();


    let data = ctx.data.read().await;
    let info = data.get::<BotInfo>().unwrap().read().await;

    let sourcemod_dir = info.get("SOURCEMOD_DIR").unwrap();
    let downloads_dir = info.get("DOWNLOADS_DIR").unwrap();
    let depot_dir = info.get("DEPOT_DIR").unwrap();

    let mut log = Vec::new();

    let mut emb = CreateEmbed::default();
    emb.title("Gamedata Checker");
    emb.thumbnail(ICON_GDC);
    emb.color(COLOR_GDC);
    log.push(String::from("Pulling latest sourcemod..."));
    emb.description(format!("```\n{}\n```", log.join("\n")));
    emb.footer(|f| f.text(format!("Requested by: {}", &msg.author.tag())));

    let mut emb_msg = discordhelpers::embed_message(emb);
    let mut message = msg.channel_id
        .send_message(&ctx.http, |_| &mut emb_msg)
        .await?;

    // grab latest sourcemod
    let sourcemod_update = Command::new("git")
        .current_dir(sourcemod_dir)
        .arg("pull")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match sourcemod_update {
        Ok(mut child) => {
            let wait = child.wait();
            if wait.is_err() {
                update_msg(&msg.author.tag(), & mut message, &ctx, &format!("SourceMod pull failed! (git exited with {})", wait.err().unwrap()), & mut log).await
            }
        }
        Err(e) => {
            update_msg(&msg.author.tag(), & mut message, &ctx, &format!("SourceMod pull failed: {}", e), & mut log).await
        }
    }

    update_msg(&msg.author.tag(), & mut message, &ctx, &format!("Downloading appid '{}'", game.appid), & mut log).await;
    let gdc = GDCManager::new(game, sourcemod_dir, downloads_dir, depot_dir);
    match gdc.download_game().await {
        Ok(t) => {
            if !t.success() {
                update_msg(&msg.author.tag(), & mut message, &ctx, &format!("Exited with status code {}", t), & mut log).await
            }
        }
        Err(e) => {
            update_msg(&msg.author.tag(), & mut message, &ctx, &format!("Fatal error: {}", e), & mut log).await
        }
    }

    update_msg(&msg.author.tag(), & mut message, &ctx, "Download completed. Running gdc...", & mut log).await;
    let mut file = OpenOptions::new()
        .write(true) // <--------- this
        .create(true)
        .open("output.log")
        .unwrap();
    let results = gdc.check_gamedata(& mut file).await;

    build_results_embed(&msg.author.tag(), &mut message, ctx, &results, & mut log).await;
    msg.channel_id.send_files(&ctx.http, vec!["output.log"], |f| {
        f
    }).await?;

    Ok(())
}

async fn build_results_embed(tag : &String, msg : &mut Message, http : &Context, results : &HashMap<String, std::result::Result<bool, GDCError>>, log : & mut Vec<String>) {
    msg.edit(http, |f| {
        f.embed(|e| {
            for (k, v) in results {
                if v.is_err() {
                    e.field(k, format!("```\n{}\n```", v.as_ref().err().unwrap()), false);
                }
            }

            e.title("Gamedata Checker");
            e.color(COLOR_GDC);
            log.push(String::from("Execution completed."));
            e.description(format!("```\n{}\n```", log.join("\n")));
            e.thumbnail(ICON_GDC);
            e.footer(|f| f.text(format!("Requested by: {}", tag)));
            e
        })
    }).await.expect("Unable to edit message.")
}

async fn update_msg(tag : &String, msg : &mut Message, http : &Context, text : &str, log : & mut Vec<String>) {

    msg.edit(http, |f| f.embed(|e| {
        e.title("Gamedata Checker");
        log.push(text.to_owned());
        e.description(format!("```\n{}\n```", log.join("\n")));
        e.thumbnail(ICON_GDC);
        e.color(COLOR_GDC);
        e.footer(|f| f.text(format!("Requested by: {}", tag)));
        e
    })).await.expect("Unable to edit message.")
}
