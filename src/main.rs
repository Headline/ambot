mod cache;
mod commands;
mod events;
mod utls;
mod notifier;
mod parser;
mod steam;

use serenity::{
    prelude::GatewayIntents,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
};

use std::{collections::HashSet, env, error::Error};

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

/** Command Registration **/
use crate::commands::{
    help::*, ping::*, gdc::*
};

#[group]
#[commands(ping, help, gdc)]
struct General;

/** Spawn bot **/
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let token = env::var("BOT_TOKEN")?;
    let http = Http::new(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            owners.insert(info.owner.id);

            if let Some(team) = info.team {
                for member in &team.members {
                    owners.insert(member.user.id);
                }
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    info!(
        "Registering owner(s): {}",
        owners
            .iter()
            .map(|o| format!("{}", o.0))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let prefix = env::var("BOT_PREFIX")?;
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&prefix))
        .before(events::before)
        .after(events::after)
        .group(&GENERAL_GROUP)
        .bucket("nospam", |b| b.delay(3).time_span(10).limit(3))
        .await
        .on_dispatch_error(events::dispatch_error);

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .await?;

    cache::fill(client.data.clone(), &prefix, &bot_id).await?;

    notifier::start_listening(client.data.clone(), client.cache_and_http.http.clone());
    steam::start_polling(client.data.clone(), client.cache_and_http.clone(), commands::gdc::on_update);

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
    Ok(())
}