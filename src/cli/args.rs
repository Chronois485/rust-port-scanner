use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "IP address or hostname")]
    pub target: String,

    #[arg(short, long, help = "Ports to scan")]
    pub ports: Option<String>,
}
