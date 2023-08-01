use crate::{Context, Error};
use redis::{AsyncCommands, RedisError};
use std::str::FromStr;
use xivapi::{
    models::character::{CharacterResult, Gender},
    prelude::{Builder, World},
};

async fn return_embed(
    method: &str,
    response: Result<CharacterResult, xivapi::error::Error>,
    ctx: &Context<'_>,
) -> Result<(), Error> {
    match response {
        Ok(r) => {
            let character = r.character.unwrap();
            let title = match &character.free_company_name {
                Some(t) => format!(
                    "[{}] {} «{}»",
                    character.active_class_job.unlocked_state.name, character.name, t
                ),
                None => format!(
                    "[{}] {}",
                    character.active_class_job.unlocked_state.name, character.name
                ),
            };

            // TODO: implement fc tag for character, currently displaying fc name as placeholder.
            // TODO: implement Display for tribe and race.
            // TODO: further cleanup the embed.
            ctx.send(|b| {
                b.embed(|e| {
                    e.title(title)
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
                            "information",
                            format!(
                                "{:?} {:?}\n{:?}",
                                character.tribe, character.race, character.gender
                            ),
                            true,
                        )
                        .field("city-state", format!("{:?}", character.town), true)
                        .field("nameday", character.nameday, false)
                        .footer(|f| f.text(format!("world: {}", character.world)))
                })
            })
            .await?;
        }
        Err(_) => {
            ctx.send(|b| {
                b.embed(|e| {
                    e.title("couldn't find your character!")
                        .description(format!(
                            "Kotonya couldn't find the character with the given {}, nya!",
                            method
                        ))
                })
            })
            .await?;
        }
    }

    Ok(())
}

/// link your character to Kotonya.
#[poise::command(slash_command)]
pub async fn link(
    ctx: Context<'_>,
    #[description = "your character name or lodestone id"] input: String,
) -> Result<(), Error> {
    let api = &ctx.data().api;
    let con = &mut ctx.data().client.get_async_connection().await?;

    ctx.defer().await?;

    match input.parse::<u64>() {
        Ok(t) => {
            let response = api.character(t.into()).send().await;

            match response {
                Ok(r) => {
                    let character = r.character.unwrap();

                    con.set(&ctx.author().id.to_string(), character.id.to_string())
                        .await?;

                    ctx.send(|b| {
                        b.embed(|e| {
                            e.title("link successful!").description(format!(
                                "successfully linked `{}` with `{}`!",
                                ctx.author().name,
                                character.name,
                            ))
                        })
                    })
                    .await?;
                }

                Err(_) => {
                    ctx.send(|b| {
                        b.embed(|e| {
                            e.title("couldn't find your character!").description(
                                "Kotonya couldn't find a character with the given ID, nya!",
                            )
                        })
                    })
                    .await?;
                }
            }
        }

        Err(_) => {
            let response = api.character_search().name(&input).send().await;

            match response {
                Ok(r) => {
                    if r.results.is_empty() {
                        ctx.send(|b| {
                            b.embed(|e| {
                                e.title("couldn't find your character!").description(
                                    "Kotonya couldn't find a character with the given name, nya!",
                                )
                            })
                        })
                        .await?;

                        return Ok(());
                    }

                    let id = r.results[0].id;
                    let character = api.character(id.into()).send().await?.character.unwrap();

                    con.set(&ctx.author().id.to_string(), character.id.to_string())
                        .await?;

                    ctx.send(|b| {
                        b.embed(|e| {
                            e.title("link successful!").description(format!(
                                "successfully linked `{}` with `{}`!",
                                ctx.author().name,
                                character.name,
                            ))
                        })
                    })
                    .await?;
                }

                Err(e) => {
                    ctx.send(|b| {
                        b.embed(|em| em.title("xivapi error!").description(format!("{:#?}", e)))
                    })
                    .await?;
                }
            }
        }
    }

    Ok(())
}

#[poise::command(slash_command, subcommands("name", "id", "_self"), subcommand_required)]
pub async fn character(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// fetch your linked character.
#[poise::command(slash_command, rename = "self")]
pub async fn _self(ctx: Context<'_>) -> Result<(), Error> {
    let id = &ctx.author().id.to_string();
    let con = &mut ctx.data().client.get_async_connection().await?;
    let result: Result<String, RedisError> = con.get(id).await;

    ctx.defer().await?;

    match result {
        Ok(t) => {
            let api = &ctx.data().api;
            let response = api.character(t.parse::<u64>().unwrap().into()).send().await;

            return_embed("ID", response, &ctx).await?;
        }
        Err(_) => {
            ctx.send(|b| b.embed(|e| e
                .title("couldn't fetch your character!")
                .description("you don't have a character linked to your Discord account. please use `/link <name/id>` to link your character!"))
            ).await?;
        }
    }

    Ok(())
}

/// fetch a character by their name and world.
#[poise::command(slash_command)]
pub async fn name(
    ctx: Context<'_>,
    #[description = "the character's name"] name: String,
    #[description = "the character's world"] world: String,
) -> Result<(), Error> {
    let api = &ctx.data().api;
    let world = World::from_str(&world);

    ctx.defer().await?;

    match world {
        Ok(w) => {
            let response = api
                .character_search()
                .name(&name)
                .server(w)
                .send()
                .await
                .unwrap();

            let id = response.results[0].id;
            let character = api.character(id.into()).send().await;

            return_embed("name", character, &ctx).await?
        }
        Err(_) => {
            ctx.send(|b| b.embed(|e| e.title("invalid world!"))).await?;
            return Ok(());
        }
    }

    Ok(())
}

/// fetch a character by their Lodestone ID.
#[poise::command(slash_command)]
pub async fn id(
    ctx: Context<'_>,
    #[description = "the character's Lodestone ID"] id: String,
) -> Result<(), Error> {
    let api = &ctx.data().api;

    ctx.defer().await?;

    let response = api
        .character(id.parse::<u64>().unwrap().into())
        .send()
        .await;

    return_embed("ID", response, &ctx).await?;

    Ok(())
}
