use crate::{Context, Error};
use redis::{AsyncCommands, RedisError};
use std::str::FromStr;
use xivapi::{
    models::free_company::FreeCompanyResult,
    prelude::{Builder, World},
};

async fn return_embed(
    method: &str,
    response: Result<FreeCompanyResult, xivapi::error::Error>,
    ctx: &Context<'_>,
) -> Result<(), Error> {
    match response {
        Ok(r) => {
            let fc = r.free_company.unwrap();
            ctx.send(|b| {
                b.embed(|e| {
                    e.title(format!("{} «{}»", fc.name, fc.tag))
                        .description(format!("Lodestone ID: `{}`\n```{}```", fc.id.0, fc.slogan))
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
            .await?;
        }
        Err(_) => {
            ctx.send(|b| {
                b.embed(|e| {
                    e.title("couldn't find your free company!")
                        .description(format!(
                            "Kotonya couldn't find the free company with the specified {}, nya!",
                            method
                        ))
                })
            })
            .await?;
        }
    }

    Ok(())
}

#[poise::command(
    rename = "freecompany",
    slash_command,
    subcommands("name", "id", "_self"),
    subcommand_required
)]
pub async fn free_company(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// fetch your linked character's free company.
#[poise::command(rename = "self", slash_command)]
pub async fn _self(ctx: Context<'_>) -> Result<(), Error> {
    let id = &ctx.author().id.to_string();
    let con = &mut ctx.data().client.get_async_connection().await?;
    let result: Result<String, RedisError> = con.get(id).await;

    ctx.defer().await?;

    match result {
        Ok(t) => {
            let api = &ctx.data().api;
            let response = api.character(t.parse::<u64>().unwrap().into()).send().await;

            match response {
                Ok(r) => {
                    let fc = r.character.unwrap().free_company_id;

                    match fc {
                        Some(f) => {
                            let response = api.free_company(f).send().await;

                            return_embed("ID", response, &ctx).await?;
                        }

                        None => {
                            ctx.send(|b| {
                                b.embed(|e| {
                                    e.title("couldn't fetch your free company!").description(
                                        "your linked character is currently not in a free company!",
                                    )
                                })
                            })
                            .await?;
                        }
                    }
                }
                Err(e) => {
                    ctx.send(|b| {
                        b.embed(|em| em.title("xivapi error!").description(format!("{:#?}", e)))
                    })
                    .await?;
                }
            }
        }
        Err(_) => {
            ctx.send(|b| b.embed(|e| e
                .title("couldn't fetch your free company!")
                .description("you don't have a character linked to your Discord account. please use `/link <name/id>` to link your character!"))
            ).await?;
        }
    }

    Ok(())
}

/// fetch a free company by its name and world.
#[poise::command(slash_command)]
pub async fn name(
    ctx: Context<'_>,
    #[description = "the free company's name"] name: String,
    #[description = "the free company's world"] world: String,
) -> Result<(), Error> {
    let api = &ctx.data().api;
    let world = World::from_str(&world);

    ctx.defer().await?;

    match world {
        Ok(w) => {
            let response = api
                .free_company_search()
                .name(&name)
                .server(w)
                .send()
                .await
                .unwrap();

            let id = response.results[0].id;
            let fc = api.free_company(id).send().await;

            return_embed("name", fc, &ctx).await?
        }
        Err(_) => {
            ctx.send(|b| b.embed(|e| e.title("invalid world!"))).await?;
            return Ok(());
        }
    }

    Ok(())
}

/// fetch a free company by its Lodestone ID.
#[poise::command(slash_command)]
pub async fn id(
    ctx: Context<'_>,
    #[description = "the free company's Lodestone ID"] id: String,
) -> Result<(), Error> {
    let api = &ctx.data().api;

    ctx.defer().await?;

    let response = api
        .free_company(id.parse::<u64>().unwrap().into())
        .send()
        .await;

    return_embed("ID", response, &ctx).await?;

    Ok(())
}
