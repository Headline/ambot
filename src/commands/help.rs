use serenity::{
    builder::CreateEmbed,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

use crate::utls::constants::*;
use crate::utls::discordhelpers;

#[command]
pub async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if !args.is_empty() {
        let cmd = args.parse::<String>().unwrap();
        let mut emb = CreateEmbed::default();
        emb.thumbnail(ICON_HELP);

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
                emb.title("Command not found");
                emb.color(COLOR_FAIL);
                emb.thumbnail(ICON_FAIL);
                unknown.as_str()
            }
        };

        emb.description(description);

        let mut emb_msg = discordhelpers::embed_message(emb);
        msg.channel_id
            .send_message(&ctx.http, |_| &mut emb_msg)
            .await?;

        return Ok(());
    }

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.thumbnail(ICON_HELP);
            e.description("I currently have no commands, add one! ");
            e.color(COLOR_OKAY);
            e.title("Commands");
// example
//            e.field("invite", "``` Grabs the bot's invite link ```", false);
            e
        })
    }).await?;

    debug!("Command executed");
    Ok(())
}
