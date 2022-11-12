use crate::cache::Cache;
use crate::mirror::Mirror;
use crate::mirrors::MirrorMeta;
use anyhow::Result;
use chrono::Utc;
use std::time::Duration;

const MIRRORS_STATUS_URL: &str = "https://archlinux.org/mirrors/status/json/";
const DEFAULT_CONNECTION_TIMEOUT: u64 = 30;
const DB_SUBPATH: &str = "community/os/x86_64/community.db";

pub struct Client {
    http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Client> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_CONNECTION_TIMEOUT))
            .build()?;

        Ok(Self { http_client })
    }
    pub async fn fetch_mirror_list(&self) -> Result<MirrorMeta> {
        let mut meta = self
            .http_client
            .get(MIRRORS_STATUS_URL)
            .send()
            .await?
            .json::<MirrorMeta>()
            .await?;

        meta.cache_time = Some(Utc::now());
        Cache::write(&meta).await;
        Ok(meta)
    }

    pub async fn fetch_db(self, mirror: &Mirror) -> Result<()> {
        let data = self
            .http_client
            .get(format!("{}/{}", mirror.url, DB_SUBPATH))
            .send()
            .await?
            .bytes()
            .await?;

        dbg!(data);
        Ok(())
    }
}
