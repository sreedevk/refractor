#![allow(dead_code)]

mod cli;
mod mirrors;
mod cache;
mod remote;
mod app;
mod benchmark;
mod mirror;

use mirrors::MirrorMeta;
use anyhow::Result;
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let mirror_list = MirrorMeta::fetch().await?;
    App::start(&mirror_list);
    Ok(())
}
