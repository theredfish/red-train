use clap::Parser;

/// Port scanner
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct PortScannerArgs {
    /// The domain under scan, ex : www.duckduckgo.com
    #[clap(short, long)]
    pub domain: String,

    /// A list of one or more ports to scan.
    /// If empty the most common ports will be scanned.
    #[clap(short, long)]
    pub port: Option<Vec<u16>>,
}
