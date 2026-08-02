mod cli;
mod ports;
mod scanner;

use clap::Parser;
use cli::args::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    let scan = scanner::tcp::connect_to_port(
        args.target.as_str(),
        match args.ports {
            Some(port) => port.parse().unwrap(),
            None => 8080,
        },
    )
    .await;
    println!("{:?}", scan)
}
