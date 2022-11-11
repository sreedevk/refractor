use chrono::Duration;
use serde::{Deserialize, Serialize};
use crate::benchmark::Benchmark;

#[derive(Serialize, Debug, Deserialize)]
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

impl Mirror {
}
