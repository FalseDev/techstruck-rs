use crate::{Context, Error};
use poise::command;

#[command(prefix_command)]
pub(crate) async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
