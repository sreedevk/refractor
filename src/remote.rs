use crate::cache::Cache;
use crate::mirror::Mirror;
use crate::mirrors::MirrorMeta;
use anyhow::Result;
use bytes::Bytes;
use chrono::Utc;
use std::time::Duration;

const MIRRORS_STATUS_URL: &str = "https://archlinux.org/mirrors/status/json/";
const DEFAULT_CONNECTION_TIMEOUT: u64 = 5;
const DEFAULT_DOWNLOAD_TIMEOUT: u64 = 30;
const DB_SUBPATH: &str = "community/os/x86_64/community.db";

pub struct Client {
    http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Client> {
        let http_client = reqwest::Client::builder()
            .build()?;

        Ok(Self { http_client })
    }
    pub async fn fetch_mirror_list(&self) -> Result<MirrorMeta> {
        let mut meta = self
            .http_client
            .get(MIRRORS_STATUS_URL)
            .timeout(Duration::from_secs(DEFAULT_CONNECTION_TIMEOUT))
            .send()
            .await?
            .json::<MirrorMeta>()
            .await?;

        meta.cache_time = Some(Utc::now());
        Cache::write(&meta).await;
        Ok(meta)
    }

    pub async fn fetch_db(&self, mirror: &Mirror) -> Result<Bytes> {
        let req = self.http_client
            .get(format!("{}{}", mirror.url, DB_SUBPATH))
            .timeout(Duration::from_secs(DEFAULT_DOWNLOAD_TIMEOUT));

        let data = req.send()
            .await?
            .bytes()
            .await?;

        Ok(data)
    }
}
