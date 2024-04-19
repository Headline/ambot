use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::utls::constants::*;
use crate::utls::discordhelpers;
use crate::utls::discordhelpers::dispatch_embed;

#[command]
pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !args.is_empty() {
        let cmd = args.parse::<String>().unwrap();
        let mut emb = CreateEmbed::default()
            .thumbnail(ICON_HELP);

        let unknown = format!("Unknown command '{}'", cmd);
        let description = match cmd.as_str() {
            "help" => "Do you like recursion or something?",
// example
//            "invite" => {
//                emb.title("Invite command");
//                emb.field("Example", format!("{}invite", prefix), false);
//                "Grabs the bot's invite link\n\n"
//            }
            _ => {
                emb = emb.title("Command not found")
                    .color(COLOR_FAIL)
                    .thumbnail(ICON_FAIL);
                unknown.as_str()
            }
        };

        emb = emb.description(description);

        discordhelpers::dispatch_embed(&ctx.http, msg.channel_id, emb).await?;

        return Ok(());
    }

    let embed = CreateEmbed::new()
        .thumbnail(ICON_HELP)
        .description("I currently have no commands, add one! ")
        .color(COLOR_OKAY)
        .title("Commands");

    dispatch_embed(&ctx.http, msg.channel_id, embed).await?;

    debug!("Command executed");
    Ok(())
}
