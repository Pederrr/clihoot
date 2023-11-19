use actix::{Actor, Context, Message};
use actix_rt::System;
use actix::prelude::*;
use async_trait::async_trait;
use futures::future::join;
use futures::{SinkExt, StreamExt, TryStreamExt};
use tokio_tungstenite::{accept_async, connect_async};
use anyhow::Result;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::{accept, connect};

pub async fn spawn_server() -> Result<()> {
    let addr = "127.0.0.1:6000";
    let try_socket = tokio::net::TcpListener::bind(addr).await;
    let listener = try_socket.expect("SERVER Failed to bind");
    println!("SERVER Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {

        let mut ws_stream = accept_async(stream).await.expect("SERVER Failed to accept");

        let msg = ws_stream.next().await.unwrap().unwrap();
        let msg_str = msg.to_text().unwrap();
        println!("SERVER got message {}", msg_str);

        ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(msg_str.to_string())).await.unwrap();
    }
    Ok(())
}