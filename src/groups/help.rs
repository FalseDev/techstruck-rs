use crate::{Context, Error};
use crate::util::ansi::{ansi,TextColors,BgColors,Style};

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub(crate) async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}


