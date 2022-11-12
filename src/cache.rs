use crate::mirrors::MirrorMeta;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use log::{info, warn};
use std::{env, fs, path::PathBuf};
use thiserror::Error;

const LOG_TARGET: &str = "refractor_cache";

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("invalid_cache: cache is stale!")]
    InvalidCache,
    #[error("invalid_cache: cache not found!")]
    CacheNotFound,
}

pub struct Cache;

impl Cache {
    pub async fn write(mirror_meta: &MirrorMeta) {
        if let Ok(serialized_cache) = serde_json::to_string(mirror_meta) {
            match fs::write(Self::cache_path(), serialized_cache) {
                Ok(_) => info!(target: LOG_TARGET, "cache written successfully!"),
                Err(e) => warn!("{}: cache write failed! {}", LOG_TARGET, e),
            }
        }
    }

    pub async fn read() -> Result<MirrorMeta> {
        let raw_cache = fs::read_to_string(Self::cache_path())?;
        let parsed_cache: MirrorMeta = serde_json::from_str(raw_cache.as_str())?;

        if Self::is_cache_valid(&parsed_cache.cache_time) {
            info!("{}: cache hit!", LOG_TARGET);
            Ok(parsed_cache)
        } else {
            warn!("{}: cache miss!", LOG_TARGET);
            Err(anyhow!(CacheError::InvalidCache))
        }
    }

    fn cache_path() -> PathBuf {
        let base_path = match env::var("XDG_CACHE_HOME") {
            Ok(cache_home) => PathBuf::from(cache_home),
            Err(_) => Self::home_dir().join(".cache"),
        };

        base_path.join("mirrorlist.json")
    }

    fn home_dir() -> PathBuf {
        match dirs::home_dir() {
            Some(dir) => dir,
            None => match env::var("HOME") {
                Ok(home) => PathBuf::from(home),
                Err(_e) => PathBuf::from(r"/home/").join(env::var("USER").unwrap()),
            },
        }
    }

    pub fn is_cache_valid(cache_time: &Option<DateTime<Utc>>) -> bool {
        !cache_time.is_none() && (Utc::now() - cache_time.unwrap()) <= Duration::seconds(3600)
    }
}
