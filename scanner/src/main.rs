use types::domain::Subdomain;

mod cli;
use clap::Parser;

mod ports;
mod types;

fn main() {
    let args = cli::PortScannerArgs::parse();
    let subdomain = Subdomain::new(args.domain);

    let subdomain = match args.port {
        None => ports::scanner::scan_common_ports(subdomain),
        Some(ports) => ports::scanner::scan_ports(subdomain, ports),
    };

    println!("{:?}", &subdomain);
}
