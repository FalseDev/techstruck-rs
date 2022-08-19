use crate::common::{Context, Error, ErrorContext};
use poise::command;
use std::{
    collections::HashMap,
    io::{prelude::Read, BufRead, Cursor},
    sync::{Arc, RwLock},
};

use serde::Deserialize;

type RtfmValue = (HashMap<String, String>, ngrammatic::Corpus);
type RtfmMapping = HashMap<String, RtfmValue>;

struct SphinxInventory {
    cursor: Cursor<bytes::Bytes>,
}

impl SphinxInventory {
    fn read_line(&mut self) -> Result<String, Error> {
        let mut line = String::new();
        self.cursor.read_line(&mut line)?;
        Ok(line.trim_end().to_owned())
    }

    fn read_compressed_chunks(self) -> Result<String, Error> {
        let mut decompressor = flate2::read::ZlibDecoder::new(self.cursor);
        let mut s = String::new();
        decompressor
            .read_to_string(&mut s)
            .context("Failed to decompress zlib encoded Sphinx inventory")?;
        Ok(s)
    }
    fn read_compressed_lines(self) -> Result<Vec<String>, Error> {
        Ok(self
            .read_compressed_chunks()?
            .lines()
            .map(ToOwned::to_owned)
            .collect())
    }

    /// Read file headers and return ``project name``
    fn process_headers(&mut self) -> Result<String, Error> {
        let version_header = self.read_line()?;
        if version_header != "# Sphinx inventory version 2" {
            anyhow::bail!("Invalid Sphinx version header: {}", version_header);
        };

        let proj_name = self.read_line()?[11..].to_owned();
        let _version = &self.read_line()?[11..];

        let encoding_header = self.read_line()?;
        if !encoding_header.contains("zlib") {
            anyhow::bail!("Invalid Sphinx encoding header: {}", encoding_header);
        }

        Ok(proj_name)
    }

    /// Process inventory and convert to a HashMap of name -> link
    fn into_map(
        mut self,
        url: &str,
        ignore_namespaces: Option<&Vec<String>>,
    ) -> Result<HashMap<String, String>, Error> {
        let _proj_name = self.process_headers()?;
        let entry_regex = regex::Regex::new(r"(?x)(.+?)\s+(\S*:\S*)\s+(-?\d+)\s+(\S+)\s+(.*)")?;
        let mut result: HashMap<String, String> = HashMap::new();
        for item in self.read_compressed_lines()? {
            let capture: Vec<&str> = entry_regex
                .captures(&item)
                .with_context(|| {
                    format!("Line does not match Sphinx inventory format: {:?}", item)
                })?
                .iter()
                .skip(1)
                .map(|r| r.map(|r| r.as_str()))
                .collect::<Option<Vec<&str>>>()
                .with_context(|| {
                    format!(
                        "Some capture groups didn't participate in matching {:?}",
                        item
                    )
                })?;
            let (name, directive, _prio, mut location, dispname) = (
                capture[0],
                capture[1],
                capture[2],
                capture[3].to_string(),
                capture[4],
            );
            let (domain, mut subdirective) = directive
                .split_once(':')
                .with_context(|| format!("Invalid sphinx directive: {:?}", directive))?;
            if directive == "py:module" && result.get(name).is_some() {
                // From the Sphinx Repository:
                // due to a bug in 1.1 and below,
                // two inventory entries are created
                // for Python modules, and the first
                // one is correct
                continue;
            }

            if directive == "std:doc" {
                subdirective = "label";
            }
            if location.ends_with('$') {
                location = location[..location.len() - 1].to_owned() + name;
            }
            let mut key = {
                if dispname == "-" {
                    name.to_owned()
                } else {
                    dispname.to_owned()
                }
            };
            let prefix = {
                if domain == "std" {
                    subdirective.to_owned() + ":"
                } else {
                    String::new()
                }
            };
            if let Some(namespaces) = ignore_namespaces {
                namespaces
                    .iter()
                    .for_each(|namespace| key = key.replace(namespace, ""))
            };

            result.insert(prefix + &key, url.to_owned() + &location);
        }
        Ok(result)
    }
}

#[derive(Deserialize)]
pub(crate) struct RtfmData {
    targets: HashMap<String, String>,
    url_overrides: HashMap<String, String>,
    ignore_namespaces: HashMap<String, Vec<String>>,

    #[serde(rename = "aliases")]
    aliases_raw: Option<HashMap<String, Vec<String>>>,
    #[serde(skip)]
    aliases: Option<HashMap<String, String>>,

    #[serde(skip)]
    rtfm_mapping: Arc<RwLock<RtfmMapping>>,
}

impl RtfmData {
    pub(crate) fn from_file(filename: &str) -> Self {
        serde_json::from_slice::<RtfmData>(&std::fs::read(filename).unwrap())
            .unwrap()
            .make_alias_map()
    }

    pub(crate) fn make_alias_map(mut self) -> Self {
        let aliases = self
            .aliases_raw
            .take()
            .unwrap()
            .into_iter()
            .flat_map(|(k, v)| v.into_iter().map(move |alias| (alias, k.clone())))
            .collect();
        self.aliases = Some(aliases);
        self
    }

    fn alias_map(&self) -> &HashMap<String, String> {
        self.aliases.as_ref().unwrap()
    }

    async fn ensure_rtfm(&self, name: &str, ctx: Context<'_>) -> Result<(), Error> {
        // Skip if already exists
        if self.rtfm_mapping.read().unwrap().get(name).is_some() {
            return Ok(());
        }
        let _typing = ctx.defer_or_broadcast().await?;
        let (object_url, mut base_url) = {
            (
                self.url_overrides.get(name).map(String::to_owned),
                self.targets.get(name).unwrap().to_owned(),
            )
        };
        // Ensure url ends with "/"
        if !base_url.ends_with('/') {
            base_url += "/";
        }

        let bytes = ctx
            .data()
            .http_client
            .get(&object_url.unwrap_or(base_url.to_owned() + "objects.inv"))
            .send()
            .await?
            .bytes()
            .await?;
        let doc_to_link = SphinxInventory {
            cursor: Cursor::new(bytes),
        }
        .into_map(&base_url, self.ignore_namespaces.get(name))?;
        let corpus = ngrammatic::CorpusBuilder::new()
            .arity(2)
            .fill(doc_to_link.keys())
            .pad_full(ngrammatic::Pad::Auto)
            .finish();

        let out = (doc_to_link, corpus);
        self.rtfm_mapping
            .write()
            .unwrap()
            .insert(name.to_string(), out);
        Ok(())
    }
}

#[command(prefix_command, slash_command)]
pub(crate) async fn rtfm(ctx: Context<'_>, name: String, query: String) -> Result<(), Error> {
    let name = match ctx.data().rtfm_data.alias_map().get(&name) {
        Some(name) => name,
        None => {
            ctx.say("Invalid name").await?;
            return Ok(());
        }
    };

    ctx.data().rtfm_data.ensure_rtfm(name, ctx).await?;
    let results = {
        let reader = ctx.data().rtfm_data.rtfm_mapping.read().unwrap();
        let (doc_to_link, corpus) = reader.get(name).unwrap();

        corpus
            .search(&query, 0.1)
            .iter()
            .map(|r| {
                (
                    r.text.to_owned(),
                    doc_to_link.get(&r.text).unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>()
    };
    ctx.send(|msg| {
        if results.is_empty() {
            msg.content("No matches.")
        } else {
            msg.embed(|embed| {
                embed.title(name).color(0x8f18a3).description(
                    results
                        .iter()
                        .map(|r| format!("[`{}`]({})", r.0, r.1))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            })
        }
    })
    .await?;
    Ok(())
}
