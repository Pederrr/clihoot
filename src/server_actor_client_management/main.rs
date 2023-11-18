mod lobby;
mod messages;
mod websocket;
use actix::Actor;
use hyper::service::service_fn;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

use std::net::SocketAddr;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

fn main() {
    let system = actix::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_current_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap()
    });

    system.block_on(init());

    system.run().unwrap();
}

async fn init() {
    // build our application with some routes
    let app = Router::new().route("/ws", get(ws_handler));

    let service = app.into_make_service_with_connect_info::<SocketAddr>();

    // run it with hyper
    let server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .executor(tokio::spawn)
        .serve(service);
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

struct FooActor {}

impl Actor for FooActor {
    type Context = actix::Context<Self>;
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    // create and start an actor for this connection
    let ws = FooActor {};

    let _addr = ws.start();
}
