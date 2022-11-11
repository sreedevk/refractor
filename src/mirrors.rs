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
    pub cutoff: u32,
    pub last_check: String,
    pub num_checks: u32,
    pub check_frequency: u32,
    pub urls: Vec<Mirror>,
}

impl MirrorMeta {
    pub async fn fetch() -> Result<MirrorMeta> {
        match Cache::read().await {
            Ok(cache) => Ok(cache),
            Err(_e) => Client::fetch().await
        }
    }
    
    pub fn country_wise_count(&self) -> String {
        let mut table: HashMap<Vec<String>, usize> = HashMap::new();
        let mut buffer = String::from("country\tcode\tcount\n");
        
        for mirror in self.urls.iter() {
            let country = mirror.country.clone();
            let country_code = mirror.country_code.clone();
            let key = vec![country, country_code];
            
            table.entry(key).and_modify(|count| *count += 1 ).or_insert(0);
        }

        for (country_info, count) in table.iter() {
            buffer.push_str(format!("{}\t{}\n", country_info.join("\t"), count).as_str())
        }

        buffer
    }

}
