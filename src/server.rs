use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Rusteze is running on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket, addr).await;
        });
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream, addr: std::net::SocketAddr) {
    println!("New connection from {}", addr);
    let mut buf = vec![0u8; 512];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("Client {} disconnected", addr);
                return;
            }
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                print!("Received from {}: {}", addr, msg);

                if socket.write_all(&buf[..n]).await.is_err() {
                    return;
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", addr, e);
                return;
            }
        }
    }
}
