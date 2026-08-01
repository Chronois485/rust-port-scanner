use super::PortStatus;

use tokio::net::TcpSocket;

pub async fn connect_to_port(target: &str, port: u16) -> PortStatus {
    let socket = TcpSocket::new_v4().unwrap();

    let stream = socket
        .connect(format!("{target}:{port}").parse().unwrap())
        .await;

    match stream {
        Err(_) => PortStatus::Closed,
        Ok(_) => PortStatus::Open,
    }
}
