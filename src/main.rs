#![allow(dead_code)]

mod cli;
mod mirrors;

use mirrors::MirrorMeta;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _app = cli::App::start();
    let mirror_list = MirrorMeta::fetch().await?;
    Ok(())
}
