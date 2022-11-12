use crate::cache::Cache;
use crate::mirror::Mirror;
use crate::remote::Client;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type CountryMirrorTable = HashMap<Vec<String>, usize>;

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
        self.urls
            .iter()
            .map(|mirror| vec![mirror.country.clone(), mirror.country_code.clone()])
            .fold(CountryMirrorTable::new(), |mut tbl, key| {
                tbl.entry(key).and_modify(|count| *count += 1).or_insert(0);
                tbl
            })
            .iter()
            .fold(
                String::from("country\tcode\tcount\n"),
                |buff, (country_info, count)| {
                    format!("{}{}\t{}\n", buff, country_info.join("\t"), count)
                },
            )
    }
}
