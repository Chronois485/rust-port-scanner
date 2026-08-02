use super::{Port, PortStatus, TIMEOUT_TIME};

use tokio::net::TcpSocket;
use tokio::time::timeout;

pub async fn connect_to_port(target: &str, port: u16) -> Port {
    let socket = TcpSocket::new_v4().unwrap();

    let stream = timeout(
        TIMEOUT_TIME,
        socket.connect(format!("{target}:{port}").parse().unwrap()),
    )
    .await;

    let status = match stream {
        Err(_) => PortStatus::Timeout,
        Ok(Err(_)) => PortStatus::Closed,
        Ok(_) => PortStatus::Open,
    };

    Port {
        number: port,
        status,
    }
}
