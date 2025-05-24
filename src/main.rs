use std::{env, net::SocketAddr};

use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Get the address to bind to
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid address");

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_stream(stream).await;
        });
    }
}

async fn handle_stream(stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error: Unable to accept tcp stream");

    let (mut sender, mut receiver) = ws_stream.split();

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(msg)) => {
                let reversed = msg.chars().rev().collect::<String>();
                sender
                    .send(Message::Text(reversed.into()))
                    .await
                    .expect("Error: Unable to send message");
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
