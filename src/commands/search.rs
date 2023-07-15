// TODO: improve on the search results - they're very primitive right now,
// just a name and a XIVAPI ID. it'd be better if i can return the XIVAPI
// data immediately instead of having users refetching with the ID.
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
    let pages_vec = results.results.chunks(5).collect::<Vec<_>>();
    let pages = pages_vec.as_slice();

    ctx.send(|b| {
        for result in pages[current_page] {
            match result {
                SearchModel::Achievement(a)
                | SearchModel::Action(a)
                | SearchModel::Emote(a)
                | SearchModel::Enemy(a)
                | SearchModel::Fate(a)
                | SearchModel::InstanceContent(a)
                | SearchModel::Item(a)
                | SearchModel::Leve(a)
                | SearchModel::Minion(a)
                | SearchModel::Mount(a)
                | SearchModel::Npc(a)
                | SearchModel::PlaceName(a)
                | SearchModel::Quest(a)
                | SearchModel::Recipe(a)
                | SearchModel::Status(a)
                | SearchModel::Title(a)
                | SearchModel::Weather(a) => {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
        }

        b.components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| b.custom_id(&prev_button_id).emoji('◀'))
                    .create_button(|b| b.custom_id(&next_button_id).emoji('▶'))
            })
        })
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
                            match result {
                                SearchModel::Achievement(a)
                                | SearchModel::Action(a)
                                | SearchModel::Emote(a)
                                | SearchModel::Enemy(a)
                                | SearchModel::Fate(a)
                                | SearchModel::InstanceContent(a)
                                | SearchModel::Item(a)
                                | SearchModel::Leve(a)
                                | SearchModel::Minion(a)
                                | SearchModel::Mount(a)
                                | SearchModel::Npc(a)
                                | SearchModel::PlaceName(a)
                                | SearchModel::Quest(a)
                                | SearchModel::Recipe(a)
                                | SearchModel::Status(a)
                                | SearchModel::Title(a)
                                | SearchModel::Weather(a) => {
                                    b.embed(|f| {
                                        f.title(&a.name)
                                            .description(format!("ID: {}", &a.id))
                                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                                    });
                                }
                            }
                        }

                        b
                    })
            })
            .await?;
    }

    Ok(())
}

#[poise::command(
    slash_command,
    subcommands(
        "achievement",
        "action",
        "emote",
        "enemy",
        "fate",
        "instance_content",
        "item",
        "leve",
        "minion",
        "mount",
        "npc",
        "place",
        "quest",
        "recipe",
        "status",
        "title",
        "weather"
    ),
    subcommand_required
)]
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
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game action.
#[poise::command(slash_command)]
pub async fn action(
    ctx: Context<'_>,
    #[description = "the action's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Action)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("action not found!")
                    .description("Kotonya couldn't find any actions with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Action(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game emote.
#[poise::command(slash_command)]
pub async fn emote(
    ctx: Context<'_>,
    #[description = "the emote's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Emote)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("emote not found!")
                    .description("Kotonya couldn't find any emotes with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Emote(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game enemy.
#[poise::command(slash_command)]
pub async fn enemy(
    ctx: Context<'_>,
    #[description = "the action's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Enemy)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("enemy not found!")
                    .description("Kotonya couldn't find any enemies with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Enemy(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game fate.
#[poise::command(slash_command)]
pub async fn fate(
    ctx: Context<'_>,
    #[description = "the FATE's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Fate)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("fate not found!")
                    .description("Kotonya couldn't find any FATEs with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Fate(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game instanced content.
#[poise::command(slash_command)]
pub async fn instance_content(
    ctx: Context<'_>,
    #[description = "the instanced content's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::InstanceContent)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("instanced content not found!").description(
                    "Kotonya couldn't find any instanced content with the specified name, nya!",
                )
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::InstanceContent(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game item.
#[poise::command(slash_command)]
pub async fn item(
    ctx: Context<'_>,
    #[description = "the item's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Item)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("item not found!")
                    .description("Kotonya couldn't find any items with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Item(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game leve.
#[poise::command(slash_command)]
pub async fn leve(
    ctx: Context<'_>,
    #[description = "the leve's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Leve)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("leve not found!")
                    .description("Kotonya couldn't find any leve with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Leve(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game minion.
#[poise::command(slash_command)]
pub async fn minion(
    ctx: Context<'_>,
    #[description = "the minion's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Minion)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("minion not found!")
                    .description("Kotonya couldn't find any minions with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Minion(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game mount.
#[poise::command(slash_command)]
pub async fn mount(
    ctx: Context<'_>,
    #[description = "the mount's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Mount)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("mount not found!")
                    .description("Kotonya couldn't find any mounts with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Mount(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game npc.
#[poise::command(slash_command)]
pub async fn npc(
    ctx: Context<'_>,
    #[description = "the NPC's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Npc)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("NPC not found!")
                    .description("Kotonya couldn't find any NPCs with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Npc(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game place.
#[poise::command(slash_command)]
pub async fn place(
    ctx: Context<'_>,
    #[description = "the place's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::PlaceName)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("place not found!")
                    .description("Kotonya couldn't find any places with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::PlaceName(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game quest.
#[poise::command(slash_command)]
pub async fn quest(
    ctx: Context<'_>,
    #[description = "the quest's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Quest)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("quest not found!")
                    .description("Kotonya couldn't find any quests with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Quest(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game recipe.
#[poise::command(slash_command)]
pub async fn recipe(
    ctx: Context<'_>,
    #[description = "the recipe's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Recipe)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("recipe not found!")
                    .description("Kotonya couldn't find any recipes with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Recipe(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game status.
#[poise::command(slash_command)]
pub async fn status(
    ctx: Context<'_>,
    #[description = "the status's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Status)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("status not found!")
                    .description("Kotonya couldn't find any statuses with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Status(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game title.
#[poise::command(slash_command)]
pub async fn title(
    ctx: Context<'_>,
    #[description = "the title's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Title)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("title not found!")
                    .description("Kotonya couldn't find any titles with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Title(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}

/// search for an in-game weather.
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "the weather's name"] name: String,
) -> Result<(), Error> {
    let search_result = &ctx
        .data()
        .api
        .search()
        .string(&name)
        .index(xivapi::models::search::Index::Weather)
        .send()
        .await?;

    if search_result.results.is_empty() {
        ctx.send(|b| {
            b.embed(|e| {
                e.title("weather not found!")
                    .description("Kotonya couldn't find any weathers with the specified name, nya!")
            })
        })
        .await?;
    } else if search_result.results.len() > 5 {
        paginate(ctx, search_result).await?;
    } else {
        ctx.send(|b| {
            for result in &search_result.results {
                if let SearchModel::Weather(a) = result {
                    b.embed(|f| {
                        f.title(&a.name)
                            .description(format!("ID: {}", &a.id))
                            .thumbnail(format!("https://xivapi.com/{}", &a.icon))
                    });
                }
            }
            b
        })
        .await?;
    }

    Ok(())
}
