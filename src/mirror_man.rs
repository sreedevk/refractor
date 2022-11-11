const MIRRORS_STATUS_URL: &str = "https://archlinux.org/mirrors/status/json/";
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Mirror {
    url: String, // "https://mirror.aarnet.edu.au/pub/archlinux/",
    protocol: String,
    last_sync: Option<String>, // "2022-11-08T05:21:15Z",
    completion_pct: f32,
    delay: Option<u32>,
    duration_avg: Option<f32>,
    duration_stddev: Option<f32>,
    score: Option<f32>,
    active: bool,
    country: String,      // "Australia",
    country_code: String, // "AU",
    isos: bool,
    ipv4: bool,
    ipv6: bool,
    details: String, // "https://archlinux.org/mirrors/aarnet.edu.au/5/"
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MirrorIndex {
    cutoff: u32,
    last_check: String,
    num_checks: u32,
    check_frequency: u32,
    urls: Vec<Mirror>,
}

pub async fn fetch_mirrors() -> Result<MirrorIndex, reqwest::Error> {
    reqwest::get(MIRRORS_STATUS_URL).await?
        .json::<MirrorIndex>().await
}


