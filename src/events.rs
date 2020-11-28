use serenity::{
    async_trait,
    framework::{standard::macros::hook, standard::CommandResult},
    model::{
        channel::Message,
        event::ResumedEvent,
        gateway::Ready,
        guild::{Guild, GuildUnavailable},
    },
    prelude::*,
};

use crate::utls::discordhelpers;
use serenity::framework::standard::DispatchError;

pub struct Handler; // event handler for serenity


#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, _: Context, _: Guild) {
    }

    async fn guild_delete(&self, _: Context, _: GuildUnavailable) {
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
pub async fn after(ctx : &Context, msg: &Message, _: &str, command_result: CommandResult) {
    if let Err(e) = command_result {
        let emb = discordhelpers::build_fail_embed(&msg.author, &format!("{}", e));
        let mut emb_msg = discordhelpers::embed_message(emb);
        if msg
            .channel_id
            .send_message(&ctx.http, |_| &mut emb_msg)
            .await
            .is_err()
        {
            // missing permissions, just ignore...
        }
    }
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    if let DispatchError::Ratelimited(_) = error {
        let emb =
            discordhelpers::build_fail_embed(&msg.author, "You are sending requests too fast!");
        let mut emb_msg = discordhelpers::embed_message(emb);
        if msg
            .channel_id
            .send_message(&ctx.http, |_| &mut emb_msg)
            .await
            .is_err()
        {}
    }
}
