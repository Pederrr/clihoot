mod connection_acceptor;
mod lobby;
mod messages;
mod websocket;
use actix::{Actor, Addr};
use actix_web_actors::ws;
use hyper::service::service_fn;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use lobby::Lobby;
use tokio::net::{TcpListener, TcpSocket};
use uuid::Uuid;
use websocket::WsConn;

use std::{net::SocketAddr, sync::Arc};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

const ROOM: uuid::Uuid = Uuid::new_v4();

fn main() {
    let system = actix::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    });

    system.block_on(init());

    system.run().unwrap();
}

async fn init() {
    // TODO:
    //  - use tungtenite websocket library directly - no HTTP upgrading
    //  - use actix for actor management

    // spawn an actor for managing the lobby
    let lobby_actor = Arc::new(Lobby::default().start());

    // spawn task for accepting connections
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let connection_acceptor: tokio::task::JoinHandle<Result<(), anyhow::Error>> =
        tokio::spawn(accept_connections(addr, lobby_actor.clone()));
}

async fn accept_connections(addr: SocketAddr, lobby: Arc<Addr<Lobby>>) -> anyhow::Result<()> {
    // create a TCP socket listener
    let listener = TcpListener::bind(addr).await?;

    loop {
        // accept a connection
        let (socket, who) = listener.accept().await?;

        // spawn a actor for managing the connection
        let ws = WsConn::new(ROOM, lobby.clone(), socket, who);
        let _ = ws.start();
    }
}

struct FooActor {}

impl Actor for FooActor {
    type Context = actix::Context<Self>;
}
