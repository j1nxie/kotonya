use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::{Context, SerenityError},
};

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<(), SerenityError> {
    // TODO: get latency of shard
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|e| e.title("pong!").description("ping: "))
                })
        })
        .await
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("a ping command.")
}
