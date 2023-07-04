use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        application_command::CommandDataOptionValue,
        command::CommandOptionType,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::{Context, SerenityError},
};
use std::str::FromStr;
use xivapi::{builder::Builder, models::character::CharacterResult, prelude::World, XivApi};

async fn get_character_by_id(
    api: &XivApi,
    id: &str,
) -> Result<CharacterResult, xivapi::error::Error> {
    api.character(id.parse::<u64>().unwrap().into())
        .send()
        .await
}

async fn get_character_by_name(
    api: &XivApi,
    name: &str,
    world: World,
) -> Result<CharacterResult, xivapi::error::Error> {
    let id = api
        .character_search()
        .name(name)
        .server(world)
        .send()
        .await
        .unwrap()
        .results[0]
        .id;

    get_character_by_id(api, &id.to_string()).await
}

async fn return_embed(
    method: &str,
    response: Result<CharacterResult, xivapi::error::Error>,
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<(), SerenityError> {
    match response {
        Ok(r) => {
            let character = r.character.unwrap();
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|e| {
                                e.title(character.name)
                                    .description(format!(
                                        "Lodestone ID: `{:?}`\n```{}```",
                                        character.id.0, character.bio
                                    ))
                                    .url(format!(
                                        "https://na.finalfantasyxiv.com/lodestone/character/{:?}",
                                        character.id.0
                                    ))
                                    .thumbnail(character.avatar)
                                    .field(
                                        "job",
                                        character.active_class_job.unlocked_state.name,
                                        true,
                                    )
                                    .field("", "", true)
                                    .field(
                                        "race | tribe | gender",
                                        format!(
                                            "{:?}\n{:?} | {:?}",
                                            character.race, character.tribe, character.gender
                                        ),
                                        true,
                                    )
                                    .field("city-state", format!("{:?}", character.town), true)
                                    .field("", "", true)
                                    .field(
                                        "guardian",
                                        format!("{:?}", character.guardian_deity),
                                        true,
                                    )
                                    .field(
                                        "server",
                                        format!("{} | {}", character.dc, character.world),
                                        true,
                                    )
                                    .field("", "", true)
                                    .field(
                                        "free company",
                                        if let Some(fc_name) = character.free_company_name {
                                            fc_name
                                        } else {
                                            String::from("None")
                                        },
                                        true,
                                    )
                                    .field("nameday", character.nameday, false)
                            })
                        })
                })
                .await
        }
        Err(_) => {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|e| {
                                e.title("couldn't find your character!")
                                    .description(format!(
                                    "Kotonya couldn't find the character with the given {}, nya!",
                                    method
                                ))
                            })
                        })
                })
                .await
        }
    }
}

pub async fn run(
    api: &XivApi,
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<(), SerenityError> {
    let params = &command.data.options.get(0).unwrap();
    let params_length = params.options.len();

    if params_length == 1 {
        let option = params
            .options
            .get(0)
            .expect("expected character ID")
            .resolved
            .as_ref()
            .expect("expected character ID object");

        if let CommandDataOptionValue::String(id) = option {
            let response = get_character_by_id(api, id).await;

            return_embed("ID", response, command, ctx).await
        } else {
            Ok(())
        }
    } else {
        let options = params
            .options
            .get(0..2)
            .expect("expected character name and world name");

        let name_option = options
            .get(0)
            .expect("expected character name")
            .resolved
            .as_ref()
            .expect("expected character name object");

        let world_option = options
            .get(1)
            .expect("expected world name")
            .resolved
            .as_ref()
            .expect("expected world name object");

        if let (CommandDataOptionValue::String(name), CommandDataOptionValue::String(world_str)) =
            (name_option, world_option)
        {
            let world = World::from_str(world_str);

            match world {
                Ok(w) => {
                    let response = get_character_by_name(api, name, w).await;

                    return_embed("name", response, command, ctx).await
                }
                Err(_) => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.embed(|e| e.title("Invalid world name!"))
                                })
                        })
                        .await
                }
            }
        } else {
            Ok(())
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("character")
        .description("fetch a character.")
        .create_option(|option| {
            option
                .name("id")
                .description("fetch the character by their Lodestone ID")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("id")
                        .description("the character's Lodestone ID")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .name("name")
                .description("fetch the character by their name")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("name")
                        .description("the character's name")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("world")
                        .description("the character's world")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
}
