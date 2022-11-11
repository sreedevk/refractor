use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::{fs, env, path::PathBuf};
use anyhow::Result;

pub const MIRRORS_STATUS_URL: &str = "https://archlinux.org/mirrors/status/json/";
pub const LOG_TARGET: &str = "refector_mirror_man";

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
        match Self::fetch_cache().await {
            Ok(cache) => { Ok(cache) },
            Err(_e) => Self::fetch_remote().await
        }
    }
    
    async fn fetch_remote() -> Result<MirrorMeta> {
        let mut meta = reqwest::get(MIRRORS_STATUS_URL)
            .await?
            .json::<MirrorMeta>()
            .await?;

        meta.cache_time = Some(Utc::now());

        Self::write_cache(&meta).await;
        Ok(meta)
    }

    async fn fetch_cache() -> Result<MirrorMeta> {
        let raw_cache = fs::read_to_string(Self::cache_path())?;
        let parsed_cache: MirrorMeta = serde_json::from_str(&raw_cache.as_str())?;
        
        if Self::is_cache_valid(&parsed_cache.cache_time) {
            println!("{}: cache hit!", LOG_TARGET);
            Ok(parsed_cache)
        }
        else {
            println!("{}: cache miss!", LOG_TARGET);
            Self::fetch_remote().await
        }
    }

    fn is_cache_valid(cache_time: &Option<DateTime<Utc>>) -> bool {
        if cache_time.is_none() {
            return false
        }
        else {
            (Utc::now() - cache_time.unwrap()) <= Duration::seconds(3600)
        }
    }

    fn home_dir() -> PathBuf {
        match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                match env::var("HOME") {
                    Ok(home) => PathBuf::from(home),
                    Err(_e) => PathBuf::from(r"/home/").join(env::var("USER").unwrap())
                }
            }
        }
    }

    fn cache_path() -> PathBuf {
        let base_path = match env::var("XDG_CACHE_HOME") {
            Ok(cache_home) => PathBuf::from(cache_home),
            Err(_) => Self::home_dir().join(".cache")
        };

        dbg!(base_path.join("mirrorlist.json"));
        base_path.join("mirrorlist.json")
    }

    async fn write_cache(mirror_meta: &MirrorMeta) {
        if let Ok(serialized_cache) = serde_json::to_string(mirror_meta) {
            match fs::write(Self::cache_path(), serialized_cache) {
                Ok(_) => println!("{}: cache written successfully!", LOG_TARGET),
                Err(e) => eprintln!("{}: cache write failed! {}", LOG_TARGET, e)
            }
        }
    }
}
