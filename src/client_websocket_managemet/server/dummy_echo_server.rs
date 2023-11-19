use actix::prelude::*;
use actix::{Actor, Message};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;

pub async fn spawn_server() -> Result<()> {
    let addr = "127.0.0.1:6000";
    let try_socket = tokio::net::TcpListener::bind(addr).await;
    let listener = try_socket.expect("SERVER Failed to bind");
    println!("SERVER Listening on: {}", addr);

    // listen for new connections
    while let Ok((stream, _)) = listener.accept().await {
        let mut ws_stream = accept_async(stream).await.expect("SERVER Failed to accept");

        // listen for messages
        while let Ok(msg) = ws_stream.next().await.unwrap() {
            let msg_str = msg.to_text().unwrap();
            println!("SERVER got message {}", msg_str);

            ws_stream
                .send(tokio_tungstenite::tungstenite::Message::Text(
                    msg_str.to_string(),
                ))
                .await
                .unwrap();
        }
    }
    Ok(())
}
