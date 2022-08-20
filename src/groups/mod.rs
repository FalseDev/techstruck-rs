mod admin;
mod help;
mod packages;
mod run;
pub(crate) mod rtfm;
use crate::common::{Data, Error};

pub(crate) fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        packages::commands(),
        admin::commands(),
        vec![help::help(), rtfm::rtfm(), run::run()],
    ]
    .into_iter()
    .flatten()
    .collect()
}
