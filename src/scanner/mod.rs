pub mod banner;
pub mod tcp;

use futures::future::join_all;
use std::{fmt::Display, sync::Arc, time::Instant};
use tokio::{net::TcpStream, sync::Semaphore, time::Duration};

pub const TIMEOUT_TIME: Duration = Duration::from_secs(5);

pub enum ConnectResult {
    Connected(TcpStream),
    Closed,
    Timeout,
}

#[derive(Debug, PartialEq)]
pub enum PortStatus {
    Open,
    Closed,
    Timeout,
}

impl Display for PortStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortStatus::Open => write!(f, "OPEN"),
            PortStatus::Closed => write!(f, "CLOSED"),
            PortStatus::Timeout => write!(f, "TIMEOUT"),
        }
    }
}

#[derive(Debug)]
pub struct Port {
    pub number: u16,
    pub status: PortStatus,
    pub banner: Option<String>,
}

#[derive(Debug)]
pub struct ScanResult {
    pub target: String,
    pub ports: Vec<Port>,
    pub elapsed: Duration,
}

pub struct ScanStats {
    pub scanend: usize,
    pub open: usize,
    pub closed: usize,
    pub timeout: usize,
}

pub async fn scan_ports(target: &str, ports: &[u16], concurrency: u16) -> ScanResult {
    let start = Instant::now();
    let semaphore = Arc::new(Semaphore::new(concurrency as usize));

    let mut scan_stats = ScanStats {
        scanend: ports.len(),
        open: 0,
        closed: 0,
        timeout: 0,
    };

    let scans = ports.iter().copied().map(|port| {
        let semaphore = Arc::clone(&semaphore);

        async move {
            let _permit = semaphore.acquire_owned().await.unwrap();

            match tcp::connect_to_port(target, port).await {
                ConnectResult::Connected(mut stream) => {
                    scan_stats.open += 1;
                    Port {
                        number: port,
                        status: PortStatus::Open,
                        banner: banner::grab(&mut stream).await,
                    }
                }

                ConnectResult::Closed => {
                    scan_stats.closed += 1;
                    Port {
                        number: port,
                        status: PortStatus::Closed,
                        banner: None,
                    }
                }

                ConnectResult::Timeout => {
                    scan_stats.timeout += 1;
                    Port {
                        number: port,
                        status: PortStatus::Timeout,
                        banner: None,
                    }
                }
            }
        }
    });

    let ports = join_all(scans).await;

    ScanResult {
        target: target.to_string(),
        ports,
        elapsed: start.elapsed(),
    }
}
