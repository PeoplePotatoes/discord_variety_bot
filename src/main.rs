// use serde::{Deserialize, Serialize};
//mod rps;
/*
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
*/
mod commands;
use commands::{util::*, games::rps::*};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::standard::{macros::{command, group}, StandardFramework, CommandResult},
};
use std::env;

//Start text dunction
fn start(prefix: &str) {
    println!(
        "PeoplePotatoes' Variety Bot, version: {}, starting up...\nPrefix is: {}",
        env!("CARGO_PKG_VERSION"),
        prefix
    );
    // let prefix = "~";
    // println!("The set command prefix is {}", PREFIX);
    if cfg!(debug_assertions) {
        println!("Warning! This is a DEBUG build! This was not built for production.");
        println!("If you are a regular user, either build with the --release flag, or grab a binary from the releases section on the github.")
    }
}

#[group]
#[commands(about, ping, rps)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /*async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == format!("{}ping", PREFIX){
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        } /* else if msg.content == format!("{}rps", PREFIX){
            if let Err(why) = msg.channel_id.say(&ctx.http, "Who do you want to play with?").await {
                println!("Error sending message: {:?}", why);
            }
        }*/
    }*/
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
   }
}



//const PREFIX: &str = "~";
// Main Function
// TODO: Implement configuration file for token and prefix
#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    #[cfg(debug_assertions)]
    let prepref = "~d~";
    #[cfg(not(debug_assertions))]
    let prepref = "~";
    start(&prepref);
    let framework = StandardFramework::new()
    .configure(|c| c.prefix(prepref).delimiter(" "))
    .group(&GENERAL_GROUP);
    //Logs bot in
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why)
    }
}
