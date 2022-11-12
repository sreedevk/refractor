use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RefractorArgs {
    /// Limit the mirror list to the n most recently synced mirrors
    #[arg(short, long)]
    pub latest: Option<u32>,
    /// Save the output file to path
    #[arg(short, long)]
    pub save: Option<String>,
    /// Sort the mirrors by {age,rate,country,score,delay}
    #[arg(long)]
    pub sort: Option<String>,
    /// https/http
    #[arg(short, long)]
    pub protocol: Option<String>,
    /// Time in seconds to wait before connection times out
    #[arg(long, default_value_t = 5)]
    pub connection_timeout: u32,
    /// Time in seconds to wait before connection times out, Defaults to 5
    #[arg(long, default_value_t = 5)]
    pub download_timeout: u32,
    /// number of cpu threads to use
    #[arg(short, long)]
    pub theads: Option<u32>,
    #[arg(short, long)]
    pub count: Option<u32>,
    /// filter by country --country=US,CA,AE
    #[arg(long)]
    pub country: Option<String>,
    /// only allow mirrors with matching regexes to be scanned
    #[arg(short, long)]
    pub iregex: Option<String>,
    /// exclude mirrors that match the regex
    #[arg(short, long)]
    pub xregex: Option<String>,
    /// minimum score filter for mirrors
    #[arg(short, long)]
    pub minscore: Option<u32>,
    #[arg(short, long)]
    pub age: Option<u32>,
    #[arg(long)]
    pub sync_delay: Option<u32>,
    #[arg(long)]
    pub score: Option<u32>,
    #[arg(long)]
    pub completion_percent: Option<u32>,
    #[arg(long)]
    pub list_countries: bool,
    /// Time in seconds for which the mirrors cache fetched from url will be valid for
    #[arg(long)]
    pub cache_timeout: Option<u32>,
    /// url to fetch mirrors list from
    #[arg(long, default_value_t = String::from("https://archlinux.org/mirrors/status/json/"))]
    pub url: String,
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
    #[arg(long)]
    pub info: bool,
    /// only return mirrors that support isos
    #[arg(long)]
    pub isos: bool,
    /// only return mirrors that support ipv4
    #[arg(long)]
    pub ipv4: bool,
    /// only return mirrors that support ipv6
    #[arg(long)]
    pub ipv6: bool,
}
