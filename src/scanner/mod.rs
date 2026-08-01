pub mod tcp;

use tokio::time::Duration;

pub const TIMEOUT_TIME: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum PortStatus {
    Open,
    Closed,
    Timeout,
}

pub struct Port {
    pub number: u16,
    pub status: PortStatus,
}

pub struct ScanResult {
    pub target: String,
    pub ports: Vec<Port>,
}

pub fn scan() {
    println!("Scanning...");
}
