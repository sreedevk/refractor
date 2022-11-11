pub struct Client;
use crate::mirrors::MirrorMeta;
use crate::cache::Cache;
use chrono::Utc;
use anyhow::Result;

const MIRRORS_STATUS_URL: &str = "https://archlinux.org/mirrors/status/json/";

impl Client {
    pub async fn fetch() -> Result<MirrorMeta> {
        let mut meta = reqwest::get(MIRRORS_STATUS_URL)
            .await?
            .json::<MirrorMeta>()
        .await?;

        meta.cache_time = Some(Utc::now());
        Cache::write(&meta).await;
        Ok(meta)
    }
}

