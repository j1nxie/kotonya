use dotenvy::dotenv;
use serenity::{
    async_trait,
    model::prelude::{command::Command, interaction::Interaction, Ready},
    prelude::*,
};
use std::env;

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            if let Err(why) = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command, &ctx).await,
                _ => Ok(()),
            } {
                println!("cannot respond to slash command: {:#?}", why);
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::register(command))
        })
        .await;

        println!("i now have the following slash commands: {:#?}", commands)
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found.");
    let token = env::var("DISCORD_TOKEN").expect("expected a Discord token in the environment.");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("error creating client.");

    if let Err(why) = client.start().await {
        println!("client error: {:#?}", why);
    }
}
