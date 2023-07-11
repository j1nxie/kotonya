use dotenvy::dotenv;
use poise::{serenity_prelude as serenity, FrameworkOptions};
use std::env;
use tracing::{error, info};
use xivapi::XivApi;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {
    api: XivApi,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("failed to load .env file.");
    tracing_subscriber::fmt::init();

    let xivapi = match env::var("XIVAPI_TOKEN") {
        Ok(s) => {
            info!("running with a XIVAPI token!");
            XivApi::with_key(s)
        }
        Err(_) => {
            info!("running without a XIVAPI token!");
            XivApi::new()
        }
    };

    let framework = poise::Framework::builder()
        .options(FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::character::character(),
                commands::free_company::free_company(),
            ],
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        poise::FrameworkError::ArgumentParse { error, .. } => {
                            if let Some(e) = error.downcast_ref::<serenity::RoleParseError>() {
                                error!("found a RoleParseError: {:#?}", e);
                            } else {
                                error!("not a RoleParseError: {:#?}", error);
                            }
                        }
                        other => {
                            if let Err(e) = poise::builtins::on_error(other).await {
                                error!("fatal error: {}", e);
                            }
                        }
                    }
                })
            },
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").unwrap())
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { api: xivapi })
            })
        });

    framework.run().await.unwrap();

    Ok(())
}
