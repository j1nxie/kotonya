use crate::{Context, Error};

/// a ping command.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|b| b.embed(|e| e.title("pong!"))).await?;

    Ok(())
}
