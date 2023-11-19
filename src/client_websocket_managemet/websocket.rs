use actix::prelude::*;
use actix::{Actor, Context, Message};

use futures::stream::{SplitSink, SplitStream};
use futures::task::SpawnExt;
use futures::{SinkExt, StreamExt};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};

// message used for request to send something server, this message should be passed to websocket actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketMsg(pub String);

// this message is send to all subscribers
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageFromServer {
    pub(crate) content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Subscribe(pub Recipient<MessageFromServer>);

// actor which represents a gateway to the server, one can send it a request for sending a message or
// just subscribe for incoming messages
pub struct WebsocketActor {
    ws_stream_tx: Arc<
        Mutex<
            RefCell<
                SplitSink<
                    WebSocketStream<MaybeTlsStream<TcpStream>>,
                    tungstenite::protocol::Message,
                >,
            >,
        >,
    >,
    ws_stream_rx: Arc<RefCell<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    subscribers: Vec<Recipient<MessageFromServer>>,
}

impl WebsocketActor {
    pub(crate) async fn new() -> Self {
        let url = url::Url::parse("ws://localhost:6000").unwrap();
        let (ws_stream, _) = connect_async(url).await.expect("Client failed to connect");

        let (tx, rx) = ws_stream.split();

        WebsocketActor {
            ws_stream_rx: Arc::new(RefCell::new(rx)),
            ws_stream_tx: Arc::new(Mutex::new(RefCell::new(tx))),
            subscribers: vec![],
        }
    }
}

// handler for message requests from another local actors
impl Handler<WebsocketMsg> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMsg, ctx: &mut Context<Self>) {
        let ws_stream = Arc::clone(&self.ws_stream_tx);

        // TODO: maybe remove Mutex
        async move {
            println!("Client websocket actor: sending message");
            ws_stream
                .lock()
                .unwrap()
                .borrow_mut()
                .send(tokio_tungstenite::tungstenite::Message::Text(msg.0))
                .await
                .unwrap();
        }
        .into_actor(self)
        .spawn(ctx);
    }
}

impl Handler<Subscribe> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        println!("Client websocket actor: subscribing");
        self.subscribers.push(msg.0);
    }
}

impl Actor for WebsocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Websocket actor is alive");

        let ws_stream_rx = Arc::clone(&self.ws_stream_rx);

        // TODO: here we are cloning an old value, new subscribers do not get subscribed
        //  -> probably use a mutex/lock/Arc or Arc<Mutex<>> to share the subscribers
        let subscribers = self.subscribers.clone();

        async move {
            // listen for messages from server
            while let Ok(incoming_msg) = ws_stream_rx.borrow_mut().next().await.unwrap() {
                let incoming_msg_text = incoming_msg.to_text().unwrap().to_string();

                println!(
                    "message arrived, sending to all {} subscribers",
                    subscribers.len()
                );
                for sub in &subscribers {
                    sub.send(MessageFromServer {
                        content: incoming_msg_text.clone(),
                    })
                    .await
                    .unwrap();
                }
            }
            println!("Client websocket closed.");
        }
        .into_actor(self)
        .spawn(ctx);
    }
}
