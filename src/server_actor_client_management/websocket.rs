use crate::lobby::Lobby;
use crate::messages::{
    ClientActorMessage, ConnectToLobby, DisconnectFromLobby, RelayMessageToClient,
    RelayMessageToLobby, WsCloseConnection,
};
use actix::{fut, ActorContext, ActorFutureExt};
use actix::{Actor, Addr, ContextFutureSpawner, Running, WrapFuture};
use actix::{AsyncContext, Handler};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::Message;
use uuid::Uuid;

pub struct WsConn {
    room: Uuid,

    lobby_addr: Arc<Addr<Lobby>>,

    connection_id: Uuid,

    receiver: Option<SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>>,

    sender: SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>,

    reader_task: Option<JoinHandle<()>>,

    who: SocketAddr,
}

impl WsConn {
    pub async fn new(
        room: Uuid,
        lobby: Arc<Addr<Lobby>>,
        socket: TcpStream,
        who: SocketAddr,
    ) -> anyhow::Result<WsConn> {
        let socket = tokio_tungstenite::accept_async(socket).await?;

        let (sender, receiver) = socket.split();

        Ok(WsConn {
            connection_id: Uuid::new_v4(),
            room,
            lobby_addr: lobby,
            receiver: Some(receiver),
            sender,
            reader_task: None,
            who,
        })
    }
}

impl Actor for WsConn {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // this is my address
        let addr = ctx.address();

        // First tell the boss that we have a new connection
        self.lobby_addr
            .send(ConnectToLobby {
                addr: addr.clone().recipient(),
                lobby_id: self.room,
                self_id: self.connection_id,
            })
            .into_actor(self)
            // If we get a response back, then we're good to go
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);

        // in order not to move self into the closure, we need to extract the fields we need
        let who = self.who;
        let receiver = self.receiver.take().unwrap();

        // Spawn a Tokio task which will read from the socket and generate messages for this actor
        let reader_task = tokio::spawn(read_messages_from_socket(receiver, who, addr));
        self.reader_task = Some(reader_task);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if let Some(reader_task) = &self.reader_task {
            reader_task.abort();
        }

        self.lobby_addr.do_send(DisconnectFromLobby {
            id: self.connection_id,
            room_id: self.room,
        });
        Running::Stop
    }
}

async fn read_messages_from_socket<'a>(
    mut receiver: SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>,
    who: SocketAddr,
    addr: Addr<WsConn>,
) {
    while let Some(Ok(msg)) = receiver.next().await {
        println!("Received message from client: {who}");
        match msg {
            Message::Text(s) => addr.do_send(RelayMessageToLobby(s.to_string())),
            Message::Binary(b) => {
                addr.do_send(RelayMessageToLobby(String::from_utf8(b).unwrap()));
            }
            Message::Close(_) => {
                println!("Client {who} disconnected from TcpStream");

                // cannot call `ctx.stop();` because we are in another Task:
                // instead, we send a message to ourselves to stop
                addr.do_send(WsCloseConnection {});

                // also quit the loop
                return;
            }
            _ => (),
        }
    }
}

impl Handler<WsCloseConnection> for WsConn {
    type Result = ();

    fn handle(&mut self, _msg: WsCloseConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();

        // also send close message to the client
        println!("Sending close to {}...", self.who);

        let _x = self.sender.send(Message::Close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from("Goodbye"),
        })));
    }
}

impl Handler<RelayMessageToClient> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: RelayMessageToClient, _ctx: &mut Self::Context) -> Self::Result {
        // take the socket and send the message

        // TODO maybe wait?
        let future = actix::fut::wrap_future::<_, Self>(self.sender.send(Message::Text(msg.0)));

        // once the wrapped future resolves, update this actor's state
        let _update_self = future.map(|_, _, _| {});
    }
}

impl Handler<RelayMessageToLobby> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: RelayMessageToLobby, _ctx: &mut Self::Context) -> Self::Result {
        // in this function, we receive a text message from the client
        println!(
            "Received message of length '{}' from client: '{}' and relaying to lobby",
            msg.0.len(),
            self.who
        );

        // tell the lobby to send it to everyone else
        self.lobby_addr.do_send(ClientActorMessage {
            id: self.connection_id,
            msg: msg.0,
            room_id: self.room,
        });
    }
}
