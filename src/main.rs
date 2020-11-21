/*
    A silly Discord bot which gives you facts about hay.

    Based heavily on the docs here:
    https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/

    and the Serinity docs + examples
    https://github.com/serenity-rs/serenity/blob/current/examples/e01_basic_ping_bot/src/main.rs
*/
use std::env;
use rand::prelude::*;
use std::fs::*;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    utils::MessageBuilder,
    model::user::*
};

struct Handler{
    fact_num: usize,
    facts_array: json::JsonValue
}

const COMMAND: &str = "!hayfacts";

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        //println!("{:?}", msg);
        if msg.content == COMMAND {

            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);
                    return;
                },
            };
            

            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.

            // Generate a random number and index into the json array object
            let mut rng = rand::rngs::OsRng;
            let rand: usize = rng.gen_range(0, self.fact_num);

            // build response
            let smart_bot_id: serenity::model::id::UserId= UserId(777199761966497842);
        
            
            let response = MessageBuilder::new()
            .mention(&smart_bot_id)
            .push(" was mentioned")
            .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
            // let response =  format!("{}", self.facts_array["facts"][rand]);
           
            // if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            //     println!("Error sending message: {:?}", why);
            // }

            // 777199761966497842


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

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_SECRET_TOKEN")
        .expect("Expected a token in the environment");

            // Read the local JSON file containing those juicy facts
            // TODO, fix error handling
            let fstr = read_to_string("data/hayfacts.json").unwrap();
            let parsed = json::parse(&fstr).unwrap();

            // Calculate how many facts we have so the rng won't generate values outside this
            // range
            let facts_array_len = parsed["facts"].len(); // access the 'facts' sub array


            // Create a new instance of the Client, logging in as a bot. This will
            // automatically prepend your bot token with "Bot ", which is a requirement
            // by Discord for bot users.
            let mut client = Client::builder(&token)
                .event_handler(Handler {
                    facts_array: parsed,
                    fact_num: facts_array_len
                })
                .await
                .expect("Err creating client");

            // Finally, start a single shard, and start listening to events.
            //
            // Shards will automatically attempt to reconnect, and will perform
            // exponential backoff until it reconnects.
            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
}