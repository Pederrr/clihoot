use actix::fut::future::FutureWrap;

use actix::Actor;

use futures_util::stream::SplitSink;
use futures_util::{Future, SinkExt};
use tokio::sync::Mutex;

use std::sync::Arc;
use tokio::net::TcpStream;

use tungstenite::Message;

/// Returns an actor future which sends a text message `msg` to the client, using the specified `sender`.
pub fn prepare_message<T: Actor>(
    sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,
    msg: String,
) -> FutureWrap<impl Future<Output = ()>, T> {
    // https://stackoverflow.com/questions/64434912/how-to-correctly-call-async-functions-in-a-websocket-handler-in-actix-web
    prepare_explicit_message(sender, Message::Text(msg))
}

/// Returns an actor future which sends a message frame `msg` to the client, using the specified `sender`.
pub fn prepare_explicit_message<T: Actor>(
    sender: Arc<Mutex<SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>>>,
    msg: Message,
) -> FutureWrap<impl Future<Output = ()>, T> {
    // https://stackoverflow.com/questions/64434912/how-to-correctly-call-async-functions-in-a-websocket-handler-in-actix-web
    let fut = async move {
        let mut sender = sender.lock().await;
        println!("locked sender and sending message: {msg:?}");
        let _ = sender.send(msg).await;
    };

    actix::fut::wrap_future::<_, T>(fut)
}
