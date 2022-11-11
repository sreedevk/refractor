use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author,version,about)]
pub struct App {
    /// Limit the mirror list to the n most recently synced mirrors
    #[arg(short, long)]
    latest: Option<u32>,
    /// Save the output file to path
    #[arg(short, long)]
    save: Option<String>,
    /// Sort the mirrors by {age,rate,country,score,delay}
    #[arg(long)]
    sort: Option<String>,
    /// https/http
    #[arg(short, long)]
    protocol: Option<String>,
    /// Time in seconds to wait before connection times out
    #[arg(long, default_value_t = 5)]
    connection_timeout: u32,
    /// Time in seconds to wait before connection times out, Defaults to 5
    #[arg(long, default_value_t = 5)]
    download_timeout: u32,
    /// number of cpu threads to use
    #[arg(short, long)]
    theads: Option<u32>,
    #[arg(short, long)]
    count: Option<u32>,
    /// filter by country --country=US,CA,AE
    #[arg(long)]
    country: Option<String>,
    /// only allow mirrors with matching regexes to be scanned
    #[arg(short, long)]
    iregex: Option<String>,
    /// exclude mirrors that match the regex
    #[arg(short, long)]
    xregex: Option<String>,
    /// minimum score filter for mirrors
    #[arg(short, long)]
    minscore: Option<u32>,
    #[arg(short, long)]
    age: Option<u32>,
    #[arg(long)]
    sync_delay: Option<u32>,
    #[arg(long)]
    score: Option<u32>,
    #[arg(long)]
    completion_percent: Option<u32>,
    #[arg(long)]
    list_countries: bool,
    /// Time in seconds for which the mirrors cache fetched from url will be valid for
    #[arg(long)]
    cache_timeout: Option<u32>,
    /// url to fetch mirrors list from
    #[arg(long, default_value_t = String::from("https://archlinux.org/mirrors/status/json/"))]
    url: String,
    #[arg(long, default_value_t = false)]
    verbose: bool,
    #[arg(long)]
    info: bool,
    /// only return mirrors that support isos
    #[arg(long)]
    isos: bool,
    /// only return mirrors that support ipv4
    #[arg(long)]
    ipv4: bool,
    /// only return mirrors that support ipv6
    #[arg(long)]
    ipv6: bool
}

impl App {
    pub fn start() {
        let args = App::parse();
        if args.list_countries {
            
        }
    }
}
