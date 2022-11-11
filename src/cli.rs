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
    /// connection timeout
    #[arg(long)]
    connection_timeout: Option<u32>,
    /// download timeout
    #[arg(short, long)]
    download_timeout: Option<u32>,
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
    list_countries: bool
}

impl App {
    pub fn start() {
        let args = App::parse();
    }
}

// --list-countries
// --cache-timeout
// --url                /* arch package list data url */
// --verbose
// --info               /* print info */
//   --isos                Only return mirrors that host ISOs.
//   --ipv4                Only return mirrors that support IPv4.
//   --ipv6                Only return mirrors that support IPv6.
