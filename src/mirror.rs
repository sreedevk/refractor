use crate::remote::Client;
use anyhow::Result;
use bytes::Bytes;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Mirror {
    pub url: String, // "https://mirror.aarnet.edu.au/pub/archlinux/",
    pub protocol: String,
    pub last_sync: Option<String>, // "2022-11-08T05:21:15Z",
    pub completion_pct: f32,
    pub delay: Option<u32>,
    pub duration_avg: Option<f32>,
    pub duration_stddev: Option<f32>,
    pub score: Option<f32>,
    pub active: bool,
    pub country: String,      // "Australia",
    pub country_code: String, // "AU",
    pub isos: bool,
    pub ipv4: bool,
    pub ipv6: bool,
    pub details: String, // "https://archlinux.org/mirrors/aarnet.edu.au/5/"
}

#[derive(Debug, Clone)]
pub struct MirrorInfo {
    pub rate: f64,
    pub time: f64,
    pub size: f64,
    pub mirror: Mirror,
    pub success: bool,
}

impl Mirror {
    pub async fn download(&self) -> Result<Bytes> {
        let client = Client::new()?;
        client.fetch_db(&self).await
    }

    pub async fn process(mirror: Mirror) -> MirrorInfo {
        let start_time = Utc::now();
        let data = Self::download(&mirror).await;
        let end_time = Utc::now();

        match data {
            Ok(resp) => {
                let download_time = ((end_time - start_time).num_milliseconds() as f64) / 1_000.0;
                let download_size = (resp.len() as f64) / 1_000_000.0;
                let download_rate = download_size / download_time;

                MirrorInfo {
                    mirror,
                    time: download_time,
                    rate: download_rate,
                    size: download_size,
                    success: true,
                }
            }
            Err(_) => MirrorInfo {
                mirror,
                time: 0.0,
                rate: 0.0,
                size: 0.0,
                success: false,
            },
        }
    }
}
