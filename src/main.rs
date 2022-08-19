mod common;
mod groups;
use common::{get_env, Context, Data, Error};

use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: groups::commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(get_env("BOT_PREFIX", None)),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .token(get_env("DISCORD_TOKEN", None))
        .intents(serenity::GatewayIntents::all())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data::new()) }));

    framework.run().await.unwrap();
}
