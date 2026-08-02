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
    let ports = args.ports.unwrap_or_else(|| "8080".to_string());
    let ports = match parser::parse_ports(&ports) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    let scan = scanner::scan_ports(&args.target, ports.as_slice(), args.concurrency).await;
    output::printer::print_scan_result(&scan, args.verbose);
}
