use crate::{Context, Error};
use poise::command;

#[command(prefix_command)]
pub(crate) async fn run(
    ctx: Context<'_>,
    #[description = "run code"] block: poise::CodeBlock,
) -> Result<(), Error> {
    match block.language {
        Some(language) => {
            let client = piston_rs::Client::default();
            let executor = piston_rs::Executor::new()
                .set_language(&language)
                .set_version(">=0")
                .add_file(
                    piston_rs::File::default()
                        .set_name("main.rs")
                        .set_content(&block.code)
                );
            let out = client.execute(&executor).await.unwrap();
            ctx.say(format!("```{}\n{}```",out.language,out.run.output)).await?;
        }
        None => {
            ctx.say("Please specify a language").await?;
        }
    }
    return Ok(())
}