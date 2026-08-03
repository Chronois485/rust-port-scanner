use tokio::{io::AsyncReadExt, net::TcpStream, time::timeout};

use crate::scanner::TIMEOUT_TIME;

pub async fn grab(stream: &mut TcpStream) -> Option<String> {
    let mut buffer = vec![0; 1024];

    match timeout(TIMEOUT_TIME, stream.read(&mut buffer)).await {
        Ok(Ok(0)) => None,
        Ok(Ok(n)) => {
            let banner = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
            Some(banner)
        }
        _ => None,
    }
}
