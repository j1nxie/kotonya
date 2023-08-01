use crate::{Context, Error};
use std::str::FromStr;
use xivapi::{
    models::character::CharacterResult,
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
            // TODO: reformat the embed so it's cleaner
            ctx.send(|b| {
                b.embed(|e| {
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
                        .field("job", character.active_class_job.unlocked_state.name, true)
                        .field(
                            "race | tribe | gender",
                            format!(
                                "{:?}\n{:?} | {:?}",
                                character.race, character.tribe, character.gender
                            ),
                            true,
                        )
                        .field("city-state", format!("{:?}", character.town), true)
                        .field("guardian", format!("{:?}", character.guardian_deity), true)
                        .field(
                            "server",
                            format!("{} | {}", character.dc, character.world),
                            true,
                        )
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

#[poise::command(slash_command, subcommands("name", "id"), subcommand_required)]
pub async fn character(_: Context<'_>) -> Result<(), Error> {
    // TODO: allow users to bind a character to their discord id
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
    let response = api
        .character(id.parse::<u64>().unwrap().into())
        .send()
        .await;

    return_embed("ID", response, &ctx).await?;

    Ok(())
}
