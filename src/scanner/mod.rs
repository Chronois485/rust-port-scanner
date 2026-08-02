pub mod tcp;

use std::fmt::Display;

use colored::Colorize;
use tokio::time::Duration;

pub const TIMEOUT_TIME: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum PortStatus {
    Open,
    Closed,
    Timeout,
}

impl Display for PortStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortStatus::Open => write!(f, "{}", "OPEN".green()),
            PortStatus::Closed => write!(f, "{}", "CLOSED".red()),
            PortStatus::Timeout => write!(f, "{}", "TIMEOUT".yellow()),
        }
    }
}

#[derive(Debug)]
pub struct Port {
    pub number: u16,
    pub status: PortStatus,
}

#[derive(Debug)]
pub struct ScanResult {
    pub target: String,
    pub ports: Vec<Port>,
}

pub async fn scan_ports(target: &str, ports: &[u16]) -> ScanResult {
    let mut scan_result = ScanResult {
        target: target.to_string(),
        ports: Vec::with_capacity(ports.len()),
    };

    for &port in ports {
        scan_result.ports.push(Port {
            number: port,
            status: tcp::connect_to_port(target, port).await,
        });
    }

    scan_result
}
