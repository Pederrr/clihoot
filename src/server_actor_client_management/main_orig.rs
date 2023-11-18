mod lobby;
mod messages;
mod websocket;
use actix::{Actor, Addr};

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use lobby::Lobby;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{path::PathBuf, sync::Arc};

use std::net::SocketAddr;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

use crate::websocket::WsConn;

fn main() {
    // let rt = tokio::runtime::Builder::new_multi_thread()
    //     .worker_threads(1)
    //     .enable_all()
    //     .build()
    //     .unwrap();

    let system = actix::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_current_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap()
    });

    system.block_on(main2());

    system.run().unwrap();

    // rt.block_on(main2());

    // main2().await;
}

async fn main2() {
    // create and spin up a lobby
    let lobby: Addr<Lobby> = Lobby::default().start();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::filter::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    // build our application with some routes
    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", get(ws_handler))
        .route("/test", get(|| async { "Hello, World!" }))
        // logging so we can see whats going on
        .layer(Extension(Arc::new(lobby)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    let server = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await;
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
    // user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    chat_server: Extension<Arc<Addr<Lobby>>>,
) -> impl IntoResponse {
    // delay for 0 ms to prevent linter warning
    tokio::time::sleep(std::time::Duration::from_millis(0)).await;

    println!("{addr} connected.");

    let chat_server = chat_server.0.clone();

    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.

    ws.on_upgrade(move |socket| handle_socket(socket, addr, chat_server))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr, chat_server: Arc<Addr<Lobby>>) {
    // sleep for 0 ms to prevent linter warning
    tokio::time::sleep(std::time::Duration::from_millis(0)).await;

    // create and start an actor for this connection
    let ws = WsConn::new(uuid::Uuid::new_v4(), chat_server, socket, who);

    let _addr = ws.start();
}
