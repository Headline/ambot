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
use serenity::all::ApplicationId;
use serenity::all::standard::{BucketBuilder, Configuration};

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

            owners.insert(info.owner.unwrap().id);

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
            .map(|o| format!("{}", o))
            .collect::<Vec<String>>()
            .join(", ")
    );

    let app_id = env::var("APPLICATION_ID").expect("Expected application id in .env file");
    let prefix = env::var("BOT_PREFIX")?;
    let configuration = Configuration::new().owners(owners).prefix(&prefix);
    let framework = StandardFramework::new()
        .before(events::before)
        .after(events::after)
        .group(&GENERAL_GROUP)
        .bucket(
            "nospam",
            BucketBuilder::new_global().delay(3).time_span(10).limit(3),
        ).await
        .on_dispatch_error(events::dispatch_error);

    framework.configure(configuration);

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .application_id(ApplicationId::new(app_id.parse::<u64>().unwrap()))
        .await?;

    cache::fill(client.data.clone(), &prefix, &bot_id).await?;

    notifier::start_listening(client.data.clone(), client.http.clone());
    steam::start_polling(client.data.clone(), client.http.clone(), on_update);

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
    Ok(())
}