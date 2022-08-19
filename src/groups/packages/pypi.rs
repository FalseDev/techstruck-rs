use crate::{Context, Error};
use poise::{command, serenity_prelude::Timestamp};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum PypiResponse {
    PypiAPIData(PypiAPIData),
    PypiError(PypiError),
}

#[derive(Deserialize)]
struct PypiAPIData {
    info: PypiPackageInfo,
}

#[derive(Deserialize)]
struct PypiPackageInfo {
    name: String,
    author: String,
    version: String,
    home_page: String,
    summary: String,
    project_url: String,
    license: Option<String>,
    requires_python: String,
}

#[derive(Deserialize)]
struct PypiError {
    message: String,
}

/// View information on a pypi package
#[command(slash_command, prefix_command, category = "Packages")]
pub(crate) async fn pypi(
    ctx: Context<'_>,
    #[description = "Name of the package"] name: String,
) -> Result<(), Error> {
    static THUMBNAIL: &str = "https://i.imgur.com/syDydkb.png";
    let response_data: PypiResponse = ctx
        .data()
        .http_client
        .get(format!("https://pypi.org/pypi/{}/json", name))
        .send()
        .await?
        .json()
        .await?;
    let data = match response_data {
        PypiResponse::PypiAPIData(data) => data.info,
        PypiResponse::PypiError(error) => {
            ctx.say(error.message).await?;
            return Ok(());
        }
    };
    ctx.send(|msg| {
        msg.embed(|e| {
            let links = format!(
                "[Package](https://pypi.org/project/{}) [Homepage]({}) [Project]({})",
                data.name, data.home_page, data.project_url
            );
            e.title(data.name)
                .description(data.summary)
                .thumbnail(THUMBNAIL)
                .color(0xE03D29)
                .timestamp(Timestamp::now())
                .field("Version", data.version, false)
                .field("Author", data.author, false)
                .field(
                    "License",
                    data.license.as_deref().unwrap_or("Unknown"),
                    false,
                )
                .field("Python version required", data.requires_python, false)
                .field("Links", links, false)
        })
    })
    .await?;
    Ok(())
}
