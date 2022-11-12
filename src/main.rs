#![allow(dead_code)]

mod app;
mod benchmark;
mod cache;
mod cli;
mod mirror;
mod mirrors;
mod remote;

use anyhow::Result;
use app::App;
use mirrors::MirrorMeta;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mirror_list = MirrorMeta::fetch().await?;
    App::start(&mirror_list);
    Ok(())
}
