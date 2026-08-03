pub mod banner;
pub mod tcp;

use std::{fmt::Display, sync::Arc};

use colored::Colorize;
use futures::future::join_all;
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
    pub banner: Option<String>,
}

#[derive(Debug)]
pub struct ScanResult {
    pub target: String,
    pub ports: Vec<Port>,
}

pub async fn scan_ports(target: &str, ports: &[u16], concurrency: u16) -> ScanResult {
    let semaphore = Arc::new(Semaphore::new(concurrency as usize));

    let scans = ports.iter().map(|&port| {
        let semaphore = semaphore.clone();
        async move {
            let _permit = semaphore.acquire_owned().await.unwrap();

            tcp::connect_to_port(target, port).await
        }
    });

    let connection_results = join_all(scans).await;

    let mut ports = vec![];

    for (i, result) in connection_results.iter().enumerate() {
        match result {
            ConnectResult::Closed => ports.push(Port {
                number: i as u16,
                status: PortStatus::Closed,
                banner: None,
            }),
            ConnectResult::Timeout => ports.push(Port {
                number: i as u16,
                status: PortStatus::Timeout,
                banner: None,
            }),
            ConnectResult::Connected(stream) => ports.push(Port {
                number: i as u16,
                status: PortStatus::Open,
                banner: banner::grab(stream),
            }),
        }
    }

    ScanResult {
        target: target.to_string(),
        ports,
    }
}
