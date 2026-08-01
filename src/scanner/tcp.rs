use super::{PortStatus, TIMEOUT_TIME};

use tokio::net::TcpSocket;
use tokio::time::timeout;

pub async fn connect_to_port(target: &str, port: u16) -> PortStatus {
    let socket = TcpSocket::new_v4().unwrap();

    let stream = timeout(
        TIMEOUT_TIME,
        socket.connect(format!("{target}:{port}").parse().unwrap()),
    )
    .await;

    match stream {
        Err(_) => PortStatus::Timeout,
        Ok(Err(_)) => PortStatus::Closed,
        Ok(_) => PortStatus::Open,
    }
}
