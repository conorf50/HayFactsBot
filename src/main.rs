/*
    A silly Discord bot which gives you facts about hay.

    Based heavily on the docs here:
    https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/

    and the Serinity docs + examples
    https://github.com/serenity-rs/serenity/blob/current/examples/e01_basic_ping_bot/src/main.rs
*/
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

const COMMAND: &str = "!hayfacts";

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == COMMAND {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // Configure the client with your Discord bot token in the environment.
            let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

            // Create a new instance of the Client, logging in as a bot. This will
            // automatically prepend your bot token with "Bot ", which is a requirement
            // by Discord for bot users.
            let mut client = Client::builder(&token)
                .event_handler(Handler)
                .await
                .expect("Err creating client");

            // Finally, start a single shard, and start listening to events.
            //
            // Shards will automatically attempt to reconnect, and will perform
            // exponential backoff until it reconnects.
            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
        })
}

// fn get_new_fact() -> Box<dyn std::result::Result<(), dyn std::error::Error>> {
//     let fstr = read_to_string("data/hayfacts.json")?;
//     let parsed = json::parse(&fstr)?;
//     let facts_num = &parsed["facts"].len();
//     let mut rng = rand::thread_rng();
//     let rand: usize = rng.gen_range(0, facts_num);
//     println!("{}", parsed["facts"][rand]);
//     Ok(())
// }
