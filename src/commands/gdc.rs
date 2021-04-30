use std::collections::HashMap;
use std::fs::OpenOptions;

use serenity::framework::standard::{macros::command, Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::builder::CreateEmbed;

use gdcrunner::gameinfo::*;
use gdcrunner::downloader::GDCError;
use gdcrunner::{GDCManager};

use crate::utls::discordhelpers;
use crate::cache::{BotInfo, Sqlite};
use crate::utls::constants::{ICON_GDC, COLOR_GDC};
use std::process::{Command, Stdio};
use rusqlite::params;
use gdcrunner::gdcrunner::GameData;

#[command]
#[owners_only]
#[sub_commands(add, list, remove)]
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


    let list : Vec<GameData> = {
        let conn = data.get::<Sqlite>().unwrap().lock().await;
        let mut stmt = conn.prepare("SELECT url, path FROM gamedata WHERE appid = ?")?;
        let person_iter = stmt.query_map(params![game.appid], |row| {
            Ok(GameData {
                appid: game.appid,
                url: row.get(0)?,
                path: row.get(1)?,
            })
        })?;

        person_iter.map(|p| p.unwrap()).collect()
    };
    let results = gdc.check_gamedata(& mut file, list).await;

    build_results_embed(&msg.author.tag(), &mut message, ctx, &results, & mut log).await;
    msg.channel_id.send_files(&ctx.http, vec!["output.log"], |f| {
        f
    }).await?;
    std::fs::remove_file("output.log")?;
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

#[command]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 3 {
        msg.reply(&ctx, "Invalid syntax: -gdc add <appid> <url OR path> <source>").await?;
        return Ok(())
    }

    let appid = args.single::<i32>()?;
    let src_type = args.single::<String>()?;
    let src = args.single::<String>()?;

    let data = ctx.data.read().await;
    let conn = data.get::<Sqlite>().unwrap().lock().await;

    if src_type == "url" {
        conn.execute("INSERT INTO gamedata (appid, url, path) VALUES (?1, ?2, ?3)",
        params![appid, src, ""])?;
    }
    else if src_type == "path" {
        conn.execute("INSERT INTO gamedata (appid, url, path) VALUES (?1, ?2, ?3)",
                     params![appid, "", src])?;
    }
    else {
        msg.reply(&ctx, format!("Invalid source type: `{}`", src)).await?;
        return Ok(())
    }


    Ok(())
}

#[command]
pub async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() != 1 {
        msg.reply(&ctx, "Invalid syntax: -gdc list <appid>").await?;
        return Ok(())
    }

    let appid = args.parse::<i32>()?;

    let data = ctx.data.read().await;
    let conn = data.get::<Sqlite>().unwrap().lock().await;

    let list = {
        let mut stmt = conn.prepare("SELECT url, path FROM gamedata WHERE appid = ?")?;
        let person_iter = stmt.query_map(params![appid], |row| {
            Ok(GameData {
                appid,
                url: row.get(0)?,
                path: row.get(1)?,
            })
        })?;

        let mut list = String::from(format!("Sources for {}\n", appid));
        for person in person_iter {
            let p = person.unwrap();
            list.push_str(&format!(" - {}{}\n", p.url, p.path));
        }
        list
    };
    msg.reply(&ctx, format!("```\n{}\n```", list)).await?;
    Ok(())
}

#[command]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.len() != 2 {
        msg.reply(&ctx, "Invalid syntax: -gdc remove <appid> <src>").await?;
        return Ok(())
    }

    let appid = args.single::<i32>()?;
    let src = args.single::<String>()?;

    let data = ctx.data.read().await;
    let conn = data.get::<Sqlite>().unwrap().lock().await;

    {
        let mut stmt = conn.prepare("DELETE FROM gamedata WHERE appid = ? AND (url = ? OR path = ?)")?;
        stmt.execute(params![appid, src, src])?;
    };
    msg.reply(&ctx, "I probably removed what you wanted").await?;
    Ok(())
}