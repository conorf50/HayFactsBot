/*
    Based heavily on the Serenity examples

    https://github.com/serenity-rs/serenity/blob/current/examples/e14_slash_commands/src/main.rs

*/


use std::env;
use std::fs::*;
use rand::Rng;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};
struct Handler {
    fact_num: usize,
    facts_array: json::JsonValue
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // Generate a random number and get the entry at that index of the facts array
            let mut rng = rand::rngs::OsRng;
            let rand: usize = rng.gen_range(0..self.fact_num);

            let response =  format!("{}", self.facts_array["facts"][rand]);

            let content = match command.data.name.as_str() {
                "hayfacts" => response,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("hayfacts").description("Facts about hay.")
                })
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        let guild_command =
            ApplicationCommand::create_global_application_command(&ctx.http, |command| {
                command.name("hayfacts").description("Facts about hay.")
            })
            .await;

        println!("I created the following global slash command: {:#?}", guild_command);
    }
}

#[tokio::main]
async fn main(){
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Read the local JSON file containing those juicy facts
    // TODO, fix error handling
    let fstr = read_to_string("data/hayfacts.json").unwrap();
    let parsed = json::parse(&fstr).unwrap();

    // Calculate how many facts we have so the rng won't generate values outside this
    // range
    let facts_array_len = parsed["facts"].len(); // access the 'facts' sub array

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler{fact_num: facts_array_len, facts_array: parsed })
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
