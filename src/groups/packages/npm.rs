use crate::{Context, Error};
use poise::{command, serenity_prelude::Timestamp};
use serde::Deserialize;

use std::collections::HashMap;
#[derive(Deserialize)]
#[serde(untagged)]
enum NpmResponse {
    NpmData(NpmData),
    NpmError(NpmError),
}
#[derive(Deserialize)]
struct NpmData {
    name: String,
    description: String,
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
    versions: HashMap<String, NpmVersionInfo>,
    maintainers: Vec<NpmMaintainer>,
}

#[derive(Deserialize)]
struct NpmMaintainer {
    name: String,
}
#[derive(Deserialize)]
struct DistTags {
    latest: String,
}
#[derive(Deserialize)]
struct NpmVersionInfo {
    version: String,
    license: String,
    homepage: String,
    repository: Repository,
}
#[derive(Deserialize)]
struct Repository {
    url: String,
}
#[derive(Deserialize)]
struct NpmError {
    error: String,
}

/// View information on an NPM package
#[command(slash_command, prefix_command, category = "Packages")]
pub(crate) async fn npm(
    ctx: Context<'_>,
    #[description = "Name of the package"] name: String,
) -> Result<(), Error> {
    static THUMBNAIL: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/db/Npm-logo.svg/800px-Npm-logo.svg.png";
    let response_data: NpmResponse = ctx
        .data()
        .http_client
        .get(format!("https://registry.npmjs.org/{}/", name))
        .send()
        .await?
        .json()
        .await?;
    let data = match &response_data {
        NpmResponse::NpmData(data) => data,
        NpmResponse::NpmError(error) => {
            ctx.say(&error.error).await?;
            return Ok(());
        }
    };
    let version_info = &data.versions[&data.dist_tags.latest];
    ctx.send(|msg| {
        msg.embed(|e| {
            let mut repository: &str = &version_info.repository.url;
            repository = repository.strip_prefix("git+").unwrap_or(repository);
            repository = repository.strip_suffix(".git").unwrap_or(repository);
            let links = format!(
                "[Package](https://www.npmjs.com/package/{}) [Repository]({}) [Homepage]({})",
                data.name, repository, version_info.homepage
            );
            e.title(&data.name)
                .description(&data.description)
                .thumbnail(THUMBNAIL)
                .color(0xE03D29)
                .timestamp(Timestamp::now())
                .field("Version", &version_info.version, false)
                .field(
                    "Maintainers",
                    data.maintainers
                        .iter()
                        .map(|m| m.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    false,
                )
                .field("License", &version_info.license, false)
                .field("Links", links, false)
        })
    })
    .await?;
    Ok(())
}
