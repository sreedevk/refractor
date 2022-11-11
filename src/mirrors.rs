use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use chrono::{Utc, DateTime};
use crate::cache::Cache;
use crate::remote::Client;

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
pub struct MirrorMeta {
    pub cache_time: Option<DateTime<Utc>>,
    cutoff: u32,
    last_check: String,
    num_checks: u32,
    check_frequency: u32,
    urls: Vec<Mirror>,
}

impl MirrorMeta {
    pub async fn fetch() -> Result<MirrorMeta> {
        match Cache::read().await {
            Ok(cache) => Ok(cache),
            Err(_e) => Client::fetch().await
        }
    }
    
    pub fn country_wise() -> String {
        let _table: HashMap<Vec<String>, usize> = HashMap::new();
        String::from("this\tis\ta\ttable")
    }

}
