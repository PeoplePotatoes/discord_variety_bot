use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::*;
use serenity::http::client::Http;
//use serenity::collector::MessageCollectorBuilder;
use tokio::try_join;
use std::time::Duration;
#[command]
async fn rps(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let argnum = args.len();
    if argnum < 1 {
        msg.channel_id.say(&ctx.http, "You need to specify who you want to play with!").await?;
    } else if argnum > 1 {
        msg.channel_id.say(&ctx.http, "You can only play with one person!").await?;
    } else {
        if let None = parse_username(args.message()) {
            msg.channel_id.say(&ctx.http, format!("I don't think `{}` is a user. Try again.", args.message())).await?;
        } else {
            if parse_username(args.message()) == Some(msg.author.id.0) {
                msg.channel_id.say(&ctx.http, "You can't play with yourself!").await?;
            } else {
                msg.channel_id.say(&ctx.http, "You both have 30 seconds to respond!").await?;
                let p1 = parse_username(args.message()).unwrap();
                let p2 = msg.author.id.0;
                let answers = try_join!(query_rps_player(p1, &ctx), query_rps_player(p2, &ctx));
                match answers {
                    Ok(((a1, u1), (a2, u2))) => {
                        if a1 == a2 {
                            let message = MessageBuilder::new().push("Hey, ").mention(&u1).push(" and ").mention(&u2).push("!\nIt's a tie!").build();
                            msg.channel_id.say(&ctx.http, message).await?;
                        } else {
                            match a1 {
                                'r' => {
                                    match a2 {
                                        'p' => win(&u2, &u1, &ctx, &msg).await,
                                        's' => win(&u1, &u2, &ctx, &msg).await,
                                        _ => unreachable!(),
                                    }
                                }
                                'p' => {
                                    match a2 {
                                        's' => win(&u2, &u1, &ctx, &msg).await,
                                        'r' => win(&u1, &u2, &ctx, &msg).await,
                                        _ => unreachable!(),
                                    }
                                }
                                's' => {
                                    match a2 {
                                        'r' => win(&u2, &u1, &ctx, &msg).await,
                                        'p' => win(&u1, &u2, &ctx, &msg).await,
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),

                            }
                        }
                    }
                    Err((err, cause)) => {
                        match cause {
                            true => {
                                let message = MessageBuilder::new().push("No response received, ").mention(&err).push(" didn't give me a move.").build();
                                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                                    println!("There was an error sending the message: {:?}", why);
                                };
                            }
                            false => {
                                let message = MessageBuilder::new().push("I couldn't DM ").mention(&err).push(".\nI'll cancel the game.").build();
                                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                                    println!("There was an error sending the message: {:?}", why);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

// Get Rock Paper Scissors answers, will error out if response not given in set time, which is
// currently 30 seconds. If you want to change it, change it here (in the await reply) and in the
// inital recommand response. If the value is used any more, consider using a const
// Returned bool in case of error reffers to whether or not the error was caused by a timeout. If
// not, it should be safe to assume there was a failure sending the inital rock paper scissors dm
async fn query_rps_player(player_id: u64, ctx: &Context) -> Result<(char, User), (User, bool)> {
    let player_un = Http::get_user(&ctx.http, player_id).await.unwrap();
    if let Err(why) = User::direct_message(&player_un, &ctx.http, |m| { m.content("Rock, paper, or scissors?") }).await {
        println!{"There was an error sending the message: {:?}\the program should error out", why};
        return Err((player_un, false));
    }
    let mut choice = 'e';
    while choice == 'e' {
        if let Some(answer) = User::await_reply(&player_un, &ctx).timeout(Duration::from_secs(30)).await {
            choice = match answer.content.to_lowercase().as_str() {
                "rock" => 'r',
                "paper" => 'p',
                "scissors" => 's',
                _ => bad_choice(&player_un, &ctx, &answer).await
            };
        } else {
            if let Err(why) = User::direct_message(&player_un, &ctx.http, |m| { m.content("You took too long to respond! >:(") }).await {
                println!("There was an error sending the message: {:?}", why);
            };
            return Err((player_un, true));
        }
    }

    return Ok((choice,player_un));
}
async fn bad_choice(player_un: &User, ctx: &Context, ans: &Message) -> char {
    if let Err(why) = User::direct_message(&player_un, &ctx.http, |m| { m.content(format!("`{}` isnt a move!\nTry again!", ans.content)) }).await {
        println!("There was an error sending the message: {:?}",why);
    };
    'e'
}
//When called, put the winner first
async fn win(win: &User, lose: &User, ctx: &Context, msg: &Message) {
    let message = MessageBuilder::new().push("Hey, ").mention(lose).push("!\n").mention(win).push(" wins!").build();
    if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
        println!("There was an error sending the message: {:?}", why);
    };
}
