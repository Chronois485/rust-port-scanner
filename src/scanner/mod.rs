pub mod tcp;

#[derive(Debug)]
pub enum PortStatus {
    Open,
    Closed,
    Filtered,
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
