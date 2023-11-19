mod lobby;
mod messages;
mod websocket;
use actix::{Actor, Addr};

use futures_util::{SinkExt, StreamExt};
use lobby::Lobby;
use tokio::net::{TcpListener, TcpStream};

use tungstenite::Message;
use uuid::Uuid;
use websocket::WsConn;

use std::{net::SocketAddr, sync::Arc};

fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn main() {
    // spawn a dummy client
    // thread::spawn(move || {
    //     create_tokio_runtime().block_on(spawn_client("0.0.0.0:3000".parse().unwrap()))
    // });

    // let server_http2 = thread::spawn(move || {
    //     // Configure a runtime for the server that runs everything on the current thread
    //     let system = actix::System::with_tokio_rt(create_tokio_runtime);

    //     system.block_on(init());

    //     // Combine it with a `LocalSet,  which means it can spawn !Send futures...
    //     let local = tokio::task::LocalSet::new();
    //     local.block_on(system.runtime(), init());

    //     system.run().unwrap();
    // });

    // server_http2.join().unwrap();

    let system = actix::System::with_tokio_rt(create_tokio_runtime);

    system.block_on(init());

    system.run().unwrap();
}

async fn init() {
    // spawn an actor for managing the lobby
    let lobby_actor = Arc::new(Lobby::default().start());

    // spawn task for accepting connections
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let connection_acceptor =
        tokio::task::spawn_local(accept_connections(addr, lobby_actor.clone()));
}

async fn spawn_client(addr: SocketAddr) -> anyhow::Result<()> {
    // wait 2 seconds
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("spawn_client, trying to connect to: {addr:?}");
    let socket = TcpStream::connect(addr).await?;

    let (socket, x) = tokio_tungstenite::client_async("ws://localhost:3000/ws", socket).await?;

    let (mut sender, mut receiver) = socket.split();

    sender.send(Message::Text("Hello".to_string())).await?;

    while let Some(msg) = receiver.next().await {
        println!("Received a message: {msg:?}");
    }

    Ok(())
}

async fn accept_connections(addr: SocketAddr, lobby: Arc<Addr<Lobby>>) -> anyhow::Result<()> {
    // create a TCP socket listener

    let listener = TcpListener::bind(addr).await?;
    let room: uuid::Uuid = Uuid::new_v4();

    loop {
        println!("Listening on: {addr:?}, waiting to accept a connection");

        // accept a connection
        let (socket, who) = listener.accept().await?;

        println!("Accepted connection from: {who:?}");

        // spawn a actor for managing the connection
        let ws = WsConn::new(room, lobby.clone(), socket, who).await?;
        let _ = ws.start();
    }
}

struct FooActor {}

impl Actor for FooActor {
    type Context = actix::Context<Self>;
}
