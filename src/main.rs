use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use actix::{Actor, Context, Message};
use actix::prelude::*;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};
use anyhow::Result;
use futures::stream::{SplitSink, SplitStream};
use tokio::net::{TcpSocket, TcpStream};
use tokio_tungstenite::MaybeTlsStream;

#[derive(Message)]
#[rtype(result = "()")]
struct WebsocketMsg(String);

#[derive(Message)]
#[rtype(result = "()")]
struct MessageFromServer{
    content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe(pub Recipient<MessageFromServer>);

struct WebsocketActor {
    ws_stream_tx: Arc<Mutex<RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>,  tungstenite::protocol::Message>>>>,
    ws_stream_rx: Arc<RefCell<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    subscribers: Vec<Recipient<MessageFromServer>>
}

impl WebsocketActor {
    async fn new() -> Self {

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

impl Handler<WebsocketMsg> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMsg, ctx: &mut Context<Self>) -> () {

        let mut ws_stream = Arc::clone(&self.ws_stream_tx);

        let fut = async move {
            println!("Client websocket actor: sending message");
            ws_stream.lock().unwrap().borrow_mut().send(tokio_tungstenite::tungstenite::Message::Text(msg.0)).await.unwrap();
        };

        let actor_fut = fut.into_actor(self);
        ctx.spawn(actor_fut);
    }
}

impl Handler<Subscribe> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        self.subscribers.push(msg.0);
    }
}

impl Actor for WebsocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Websocket actor is alive");

        let mut ws_stream = Arc::clone(&self.ws_stream_rx);
        let subscribers = self.subscribers.clone();

        let fut = async move {

            while let Ok(incoming_msg) = ws_stream.borrow_mut().next().await.unwrap() {
                let incoming_msg_text = incoming_msg.to_text().unwrap().to_string();

                println!("message arrived, sending to all subscribers");
                for sub in &subscribers {
                    sub.send(MessageFromServer{content: incoming_msg_text.clone()}).await.unwrap();
                }
            }
            println!("Client websocket closed.");
        };
        let actor_fut = fut.into_actor(self);
        ctx.spawn(actor_fut);
    }
}

struct DummyClientActor {
    websocket_astor_addr: Addr<WebsocketActor>,
}

impl Actor for DummyClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Dummy client actor is alive");
    }
}

impl Handler<MessageFromServer> for DummyClientActor {
    type Result = ();

    fn handle(&mut self, msg: MessageFromServer, ctx: &mut Context<Self>) -> Self::Result {
        println!("Inside client actor handler: message from server arrive: {}", msg.content);
    }
}

fn main() -> Result<()> {
    let sys = actix::System::new();

    thread::spawn(|| {tokio::runtime::Runtime::new().unwrap().block_on(async {clihoot::server::dummy_echo_server::spawn_server().await.unwrap()})});
    thread::sleep(Duration::from_millis(1000));

    sys.block_on(async {

        // start actors
        let addr_websocket_actor = WebsocketActor::new().await.start();
        let addr_dummy = DummyClientActor {websocket_astor_addr: addr_websocket_actor.clone()}.start();

        addr_websocket_actor.send(Subscribe(addr_dummy.recipient())).await.unwrap();

        addr_websocket_actor.send(WebsocketMsg("C IS BETTER THEN RUST, CHANGE MY MIND".to_string())).await.unwrap();
    });
    sys.run()?;

    Ok(())
}