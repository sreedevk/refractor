use crate::mirror::Mirror;
use crate::remote::Client;
use crate::{cache::Cache, mirror::MirrorInfo};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, thread};
use std::io::Write;
use tabwriter::TabWriter;

type CountryMirrorTable = HashMap<Vec<String>, usize>;

pub enum SortCondition {
    Age,     // last server synchronization,
    Rate,    // download rate,
    Country, // country name, either alphabetically or in the order given by the --country option,
    Score,   // MirrorStatus score,
    Delay,   // MirrorStatus delay,
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

    async fn process(&self) -> Vec<MirrorInfo> {
        let mut mirror_info: Vec<MirrorInfo> = Vec::with_capacity(self.urls.len());
        Self::write_download_info_headers();

        for mirror in self.urls.iter() {
            let x = Mirror::process(mirror.clone()).await;
            if x.success {
                Self::write_download_info(&x);
                mirror_info.push(x);
            }
        }

        mirror_info
    }

    fn write_download_info_headers() {
        let mut tw = TabWriter::new(std::io::stdout()).padding(1);
        let out = "mirror\ttime\trate\n";
        tw.write_all(out.as_bytes()).unwrap();
        tw.flush().unwrap();
    }

    fn write_download_info(mi: &MirrorInfo) {
        let mut tw = TabWriter::new(std::io::stdout()).padding(1);
        let out = format!("{}\t{} secs\t{:.2} MB/s\n", mi.mirror.url, mi.time, mi.rate);
        tw.write_all(out.as_bytes()).unwrap();
        tw.flush().unwrap();
    }

    pub async fn sort(&self, _by: SortCondition) {
        self.process().await;
    }
}
