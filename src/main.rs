#![allow(dead_code)]

mod cli;
mod mirror_man;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let mirror_list = mirror_man::fetch_mirrors().await?;
    // dbg!(mirror_list);
    let app = cli::App::start();
    Ok(())
}
