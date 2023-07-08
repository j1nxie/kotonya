use std::str::FromStr;

use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            application_command::CommandDataOptionValue, command::CommandOptionType,
            InteractionResponseType,
        },
    },
    prelude::{Context, SerenityError},
};
use xivapi::{
    models::free_company::FreeCompanyResult,
    prelude::{Builder, World},
    XivApi,
};

async fn get_free_company_by_id(
    api: &XivApi,
    id: &str,
) -> Result<FreeCompanyResult, xivapi::error::Error> {
    api.free_company(id.parse::<u64>().unwrap().into())
        .send()
        .await
}

async fn get_free_company_by_name(
    api: &XivApi,
    name: &str,
    world: World,
) -> Result<FreeCompanyResult, xivapi::error::Error> {
    let id = api
        .free_company_search()
        .name(name)
        .server(world)
        .send()
        .await
        .unwrap()
        .results[0]
        .id;

    get_free_company_by_id(api, &id.to_string()).await
}

async fn return_embed(
    method: &str,
    response: Result<FreeCompanyResult, xivapi::error::Error>,
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<(), SerenityError> {
    match response {
        Ok(r) => {
            let fc = r.free_company.unwrap();
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|e| {
                                e.title(format!("{} «{}»", fc.name, fc.tag))
                                    .description(format!(
                                        "Lodestone ID: `{}`\n```{}```",
                                        fc.id.0, fc.slogan
                                    ))
                                    .url(format!(
                                        "https://na.finalfantasyxiv.com/lodestone/freecompany/{}",
                                        fc.id.0
                                    ))
                                    .thumbnail(fc.crest[1].clone())
                                    .field("formed", fc.formed, true)
                                    .field("", "", true)
                                    .field("grand company", fc.grand_company, true)
                                    .field("server", format!("{}", fc.server), true)
                                    .field("", "", true)
                                    .field("active member count", fc.active_member_count, true)
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
                                e.title("couldn't find the specified free company!")
                                    .description(format!(
                                "Kotonya couldn't find the free company with the given {}, nya!",
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
            .expect("expected free company ID")
            .resolved
            .as_ref()
            .expect("expected free company ID object");

        if let CommandDataOptionValue::String(id) = option {
            let response = get_free_company_by_id(api, id).await;

            return_embed("ID", response, command, ctx).await
        } else {
            Ok(())
        }
    } else {
        let options = params
            .options
            .get(0..2)
            .expect("expected free company name and world name");

        let name_option = options
            .get(0)
            .expect("expected free company name")
            .resolved
            .as_ref()
            .expect("expected free company name object");

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
                    let response = get_free_company_by_name(api, name, w).await;

                    return_embed("name", response, command, ctx).await
                }
                Err(_) => {
                    command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.embed(|e| e.title("invalid world name!"))
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
        .name("freecompany")
        .description("fetch a free company.")
        .create_option(|option| {
            option
                .name("id")
                .description("fetch the free company by their Lodestone ID")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("id")
                        .description("the free company's Lodestone ID")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        })
    // FIXME: for some unknown reason only the fetch by name subcommand doesn't work.
    // .create_option(|option| {
    //     option
    //         .name("name")
    //         .description("fetch the free company by their name")
    //         .kind(CommandOptionType::SubCommand)
    //         .create_sub_option(|option| {
    //             option
    //                 .name("name")
    //                 .description("the free company's name")
    //                 .kind(CommandOptionType::String)
    //                 .required(true)
    //         })
    //         .create_sub_option(|option| {
    //             option
    //                 .name("world")
    //                 .description("the free company's world")
    //                 .kind(CommandOptionType::String)
    //                 .required(true)
    //         })
    // })
}
