use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "IP address or hostname")]
    pub target: String,

    #[arg(short, long, help = "Ports to scan")]
    pub ports: Option<String>,

    #[arg(
        short,
        long,
        help = "Maximum amount of socets to use",
        default_value_t = 500
    )]
    pub concurrency: u16,

    #[arg(short, long, help = "Shows closed and timed out ports")]
    pub verbose: bool,
}
