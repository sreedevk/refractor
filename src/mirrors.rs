use crate::cache::Cache;
use crate::mirror::Mirror;
use crate::remote::Client;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        let client = Client::new().unwrap();
        match Cache::read().await {
            Ok(cache) => Ok(cache),
            Err(_e) => client.fetch_mirror_list().await,
        }
    }

    pub fn country_wise_count(&self) -> String {
        let mut table: HashMap<Vec<String>, usize> = HashMap::new();
        let mut buffer = String::from("country\tcode\tcount\n");

        for mirror in self.urls.iter() {
            let country = mirror.country.clone();
            let country_code = mirror.country_code.clone();

            if country.is_empty() {
                continue;
            };

            let key = vec![country, country_code];
            table
                .entry(key)
                .and_modify(|count| *count += 1)
                .or_insert(0);
        }

        for (country_info, count) in table.iter() {
            buffer.push_str(format!("{}\t{}\n", country_info.join("\t"), count).as_str())
        }

        buffer
    }
}
