// use crate::command::Command;
// use crate::db::{Db, Value};
// use std::sync::Arc;
// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
// use tokio::sync::Mutex;

pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Rusteze is running on {}", addr);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(_socket: tokio::net::TcpStream) {
    println!("New Connection")
    // TODO: handle client
}
