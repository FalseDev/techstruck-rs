use crate::{Context, Error};
use poise::{command, serenity_prelude::Timestamp};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
enum CrateAPIResponse {
    CrateAPIData(CrateAPIData),
    CrateAPIErrors(CrateAPIErrors),
}

#[derive(Deserialize)]
struct CrateAPIErrors {
    errors: Vec<CrateAPIError>,
}
#[derive(Deserialize)]
struct CrateAPIError {
    detail: String,
}

#[derive(Deserialize)]
struct CrateAPIData {
    #[serde(rename = "crate")]
    crate_: CrateInfo,
    versions: Vec<CrateVersion>,
    categories: Vec<CrateCategory>,
}

#[derive(Deserialize)]
struct CrateInfo {
    name: String,
    description: String,
    downloads: u64,
    repository: String,
    homepage: Option<String>,
    documentation: Option<String>,
}

#[derive(Deserialize)]
struct CrateVersion {
    published_by: Option<CrateAuthor>,
    num: String,
    license: Option<String>,
}

#[derive(Deserialize)]
struct CrateAuthor {
    name: Option<String>,
    login: String,
    url: String,
}

#[derive(Deserialize)]
struct CrateCategory {
    category: String,
}

/// View information on a crate in crates.io
#[command(slash_command, prefix_command, category = "Packages")]
pub(crate) async fn crates(
    ctx: Context<'_>,
    #[description = "Name of the crate"] name: String,
) -> Result<(), Error> {
    static THUMBNAIL:&str = "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png";
    let response_data: CrateAPIResponse = ctx
        .data()
        .http_client
        .get(format!("https://crates.io/api/v1/crates/{}", name))
        .send()
        .await?
        .json()
        .await?;
    let data = match response_data {
        CrateAPIResponse::CrateAPIData(api_data) => api_data,
        CrateAPIResponse::CrateAPIErrors(errors) => {
            ctx.say(&errors.errors[0].detail).await?;
            return Ok(());
        }
    };
    let crate_ = data.crate_;
    let version_data = &data.versions[1];
    ctx.send(|res| {
        res.embed(|embed| {
            let mut links = format!(
                "[Package](https://crates.io/crates/{}) [Repository]({})",
                crate_.name, crate_.repository
            );
            if let Some(documentation) = crate_.documentation {
                links += &format!(" [Documentation]({})", documentation);
            }
            if let Some(homepage) = crate_.homepage {
                links += &format!(" [Homepage]({})", homepage);
            }

            embed
                .title(crate_.name)
                .description(crate_.description)
                .thumbnail(THUMBNAIL)
                .color(0xE03D29)
                .timestamp(Timestamp::now());
            if !data.categories.is_empty() {
                embed.field(
                    "Categories",
                    data.categories
                        .iter()
                        .map(|c| c.category.to_owned())
                        .collect::<Vec<_>>()
                        .join(", "),
                    false,
                );
            }
            embed
                .field(
                    "License",
                    version_data.license.as_deref().unwrap_or("Unknown"),
                    false,
                )
                .field("Downloads", crate_.downloads, false);
            if let Some(ref author) = version_data.published_by {
                embed.field(
                    "Author",
                    format!(
                        "[{}]({})",
                        author.name.as_deref().unwrap_or(&author.login),
                        author.url
                    ),
                    false,
                );
            }
            embed
                .field("Links", links, false)
                .field("Latest Version", &version_data.num, false)
        })
    })
    .await?;
    Ok(())
}
