pub(crate) use anyhow::Context as ErrorContext;
use std::sync::Arc;

use crate::groups::rtfm::RtfmData;

pub(crate) type Error = anyhow::Error;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;

// Data which is stored and accessible in all command invocations
#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) http_client: Arc<reqwest::Client>,
    pub(crate) rtfm_data: Arc<RtfmData>,
}

impl Data {
    pub(crate) fn new() -> Self {
        let http_client = Arc::new(
            reqwest::Client::builder()
                .user_agent(get_env("USER_AGENT", None))
                .build()
                .unwrap(),
        );
        let rtfm_data = Arc::new(RtfmData::from_file(&get_env(
            "RTFM_CONFIG_FILE",
            Some("./config/rtfm.json".into()),
        )));
        Self {
            http_client,
            rtfm_data,
        }
    }
}

pub(crate) fn get_env(name: &str, default: Option<String>) -> String {
    std::env::var(name).unwrap_or_else(|err| {
        default.unwrap_or_else(|| {
            Err(err)
                .with_context(|| format!("Missing Env Variable: {:?}", name))
                .unwrap()
        })
    })
}
