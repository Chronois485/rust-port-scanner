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
}

pub async fn scan_ports(target: &str, ports: &[u16], concurrency: u16) -> ScanResult {
    let semaphore = Arc::new(Semaphore::new(concurrency as usize));

    let scans = ports.iter().copied().map(|port| {
        let semaphore = Arc::clone(&semaphore);

        async move {
            let _permit = semaphore.acquire_owned().await.unwrap();

            match tcp::connect_to_port(target, port).await {
                ConnectResult::Connected(mut stream) => Port {
                    number: port,
                    status: PortStatus::Open,
                    banner: banner::grab(&mut stream).await,
                },

                ConnectResult::Closed => Port {
                    number: port,
                    status: PortStatus::Closed,
                    banner: None,
                },

                ConnectResult::Timeout => Port {
                    number: port,
                    status: PortStatus::Timeout,
                    banner: None,
                },
            }
        }
    });

    let ports = join_all(scans).await;

    ScanResult {
        target: target.to_string(),
        ports,
    }
}
