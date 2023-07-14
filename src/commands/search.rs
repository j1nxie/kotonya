use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use xivapi::{
    models::search::{SearchModel, SearchResult},
    prelude::Builder,
};

async fn paginate<U, E>(
    ctx: poise::Context<'_, U, E>,
    results: &SearchResult,
) -> Result<(), serenity::Error> {
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx.id());
    let next_button_id = format!("{}next", ctx.id());

    let mut current_page = 0;
    let pages_vec = results.results.windows(5).collect::<Vec<_>>();
    let pages = pages_vec.as_slice();

    ctx.send(|b| {
        for result in pages[current_page] {
            if let SearchModel::Achievement(a) = result {
                b.embed(|f| {
                    f.title(&a.metadata.name)
                        .description(format!("ID: {}", &a.metadata.id))
                        .thumbnail(format!("https://xivapi.com/{}", &a.metadata.icon))
                })
                .components(|b| {
                    b.create_action_row(|b| {
                        b.create_button(|b| b.custom_id(&prev_button_id).emoji('◀'))
                            .create_button(|b| b.custom_id(&next_button_id).emoji('▶'))
                    })
                });
            }
        }

        b
    })
    .await?;

    while let Some(press) = serenity::CollectComponentInteraction::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(10))
        .await
    {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }

        press
            .create_interaction_response(ctx, |b| {
                b.kind(serenity::InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|b| {
                        for result in pages[current_page] {
                            if let SearchModel::Achievement(a) = result {
                                b.embed(|f| {
                                    f.title(&a.metadata.name)
                                        .description(format!("ID: {}", &a.metadata.id))
                                        .thumbnail(format!(
                                            "https://xivapi.com/{}",
                                            &a.metadata.icon
                                        ))
                                });
                            }
                        }

                        b
                    })
            })
            .await?;
    }

    Ok(())
}

#[poise::command(slash_command, subcommands("achievement"), subcommand_required)]
pub async fn search(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// search for an in-game achievement.
#[poise::command(slash_command)]
pub async fn achievement(
    ctx: Context<'_>,
    #[description = "the achievement's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Achievement)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("achievement not found!").description(
                    "Kotonya couldn't find any achievements with the specified name, nya!",
                )
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Achievement(a) = result {
                    b.embed(|f| {
                        f.title(&a.metadata.name)
                            .description(format!("ID: {}", &a.metadata.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.metadata.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}
