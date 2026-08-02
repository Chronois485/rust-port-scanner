mod cli;
mod output;
mod ports;
mod scanner;

use clap::Parser;
use cli::args::Args;
use ports::parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ports = parser::parse_ports(
        match args.ports {
            Some(p) => p,
            None => "8080".to_string(),
        }
        .as_str(),
    );
    let error = ports.unwrap(); // FIX LATER
    let ports = error.as_slice();
    let scan = scanner::scan_ports(&args.target, ports).await;
    output::printer::print_scan_result(&scan);
}
