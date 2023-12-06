use actix::{Actor, Addr};

use actix_rt::System;

use lobby::Lobby;
use tokio::net::TcpListener;

use uuid::Uuid;
use websocket::WsConn;

use std::{net::SocketAddr, sync::mpsc::Sender};

use super::{
    lobby::{self, Question},
    websocket,
};

fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

pub fn run_server(
    tx: Sender<Addr<Lobby>>,
    questions: Vec<Question>,
    randomize_answers: bool,
    randomize_questions: bool,
) {
    let system = actix::System::with_tokio_rt(create_tokio_runtime);

    system.block_on(init(tx, questions, randomize_answers, randomize_questions));

    system.run().unwrap();
}

#[allow(clippy::unused_async)]
async fn init(
    tx: Sender<Addr<Lobby>>,
    questions: Vec<Question>,
    randomize_answers: bool,
    randomize_questions: bool,
) {
    // spawn an actor for managing the lobby
    let lobby_actor = Lobby::new(questions, randomize_questions, randomize_answers).start();

    // spawn task for accepting connections
    // LOCAL SPAWN is very important here (actors can only be spawned on the same thread)
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let _connection_acceptor =
        tokio::task::spawn_local(accept_connections(addr, lobby_actor.clone()));

    // send the address of the lobby to the main thread
    let _ = tx.send(lobby_actor.clone());

    // handle CTRL+C gracefully
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("CTRL-C received, shutting down");
        System::current().stop();
    });
}

async fn accept_connections(addr: SocketAddr, lobby: Addr<Lobby>) -> anyhow::Result<()> {
    // create a TCP socket listener

    let listener = TcpListener::bind(addr).await?;
    let room: uuid::Uuid = Uuid::new_v4();

    loop {
        println!("Listening on: {addr:?}, waiting to accept a new connection");

        // accept a connection
        let (socket, who) = listener.accept().await?;

        println!("Accepted connection from: {who:?}");

        // spawn a actor for managing the connection
        let ws = WsConn::new(room, lobby.clone(), socket, who).await?;
        let _ = ws.start();
    }
}