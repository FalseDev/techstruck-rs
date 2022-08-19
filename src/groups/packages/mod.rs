mod crates;
mod npm;
mod pypi;

use crate::{Data, Error};

pub(crate) fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![npm::npm(), crates::crates(), pypi::pypi()]
}
