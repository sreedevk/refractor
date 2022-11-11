#![allow(dead_code)]

mod cli;
mod mirrors;
mod cache;
mod remote;

use mirrors::MirrorMeta;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    cli::App::start();
    let _mirror_list = MirrorMeta::fetch().await?;
    Ok(())
}
