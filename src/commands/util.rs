use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, format!("Cassandra's Variety bot, version {}", env!("CARGO_PKG_VERSION"))).await?;
    #[cfg(debug_assertions)]
    msg.channel_id.say(&ctx.http, "This is a DEBUG BUILD, built without runtime optimizations.\n If you are running this in a production environment, compile it with the --release flag, or download the binary from the github (if available)").await?;

    Ok(())
}
