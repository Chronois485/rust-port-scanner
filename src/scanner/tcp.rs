use super::{ConnectResult, TIMEOUT_TIME};

use tokio::net::TcpSocket;

use tokio::time::timeout;

pub async fn connect_to_port(target: &str, port: u16) -> ConnectResult {
    let socket = TcpSocket::new_v4().unwrap();

    let mut stream = timeout(
        TIMEOUT_TIME,
        socket.connect(format!("{target}:{port}").parse().unwrap()),
    )
    .await;

    match stream {
        Ok(Ok(s)) => ConnectResult::Connected(s),
        Err(_) => ConnectResult::Timeout,
        Ok(Err(_)) => ConnectResult::Closed,
    }
}
