use serenity::{async_trait, framework::{standard::macros::hook, standard::CommandResult}, model::{
    channel::Message,
    event::ResumedEvent,
    gateway::Ready,
    guild::{Guild},
    prelude::UnavailableGuild,
}, prelude::*};

use serenity::framework::standard::DispatchError;
use crate::utls::discordhelpers::{build_fail_embed, dispatch_embed};

pub struct Handler; // event handler for serenity


#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, _: Context, _: Guild, _: Option<bool>) {
    }

    async fn guild_delete(&self, _: Context, _: UnavailableGuild, _: Option<Guild>) {
    }
    async fn message(&self, _: Context, _: Message) {
    }

    async fn ready(&self, _: Context, _: Ready) {
        info!("Bot Ready");
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[hook]
pub async fn before(_: &Context, _: &Message, _: &str) -> bool {
    true
}

#[hook]
pub async fn after(
    ctx: &Context,
    msg: &Message,
    _: &str,
    command_result: CommandResult,
) {
    if let Err(e) = command_result {
        let emb = build_fail_embed(&msg.author, &format!("{}", e));
        let _ = dispatch_embed(&ctx.http, msg.channel_id, emb).await;
    }
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError, _: &str) {
    if let DispatchError::Ratelimited(_) = error {
        let emb = build_fail_embed(&msg.author, "You are sending requests too fast!");
        let _ = dispatch_embed(&ctx.http, msg.channel_id, emb).await;
    }
}