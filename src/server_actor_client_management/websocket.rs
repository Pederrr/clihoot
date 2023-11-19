use crate::lobby::Lobby;
use crate::messages::{
    ClientActorMessage, ConnectToLobby, DisconnectFromLobby, RelayMessageToClient,
    RelayMessageToLobby, WsCloseConnection,
};
use actix::{fut, ActorContext, ActorFutureExt};
use actix::{Actor, Addr, ContextFutureSpawner, Running, WrapFuture};
use actix::{AsyncContext, Handler};
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use uuid::Uuid;

pub struct WsConn {
    /// The room this connection is in
    room: Uuid,

    /// The address of the lobby - that is the actor that will handle all the messages
    lobby_addr: Arc<Addr<Lobby>>,

    /// The socket uuid for this connection
    connection_id: Uuid,

    /// The actual websocket, which can be used to send messages to client
    sender: SplitSink<WebSocket, Message>,

    /// The actual websocket, which can be used to receive messages from client
    /// There is a dedicated task (`reader_task`) which reads from this.
    receiver: Option<SplitStream<WebSocket>>,

    /// The task that reads from the websocket and sends messages to this actor
    reader_task: Option<JoinHandle<()>>,

    /// The IP address of the client
    who: SocketAddr,
}

impl WsConn {
    pub fn new(room: Uuid, lobby: Arc<Addr<Lobby>>, socket: WebSocket, who: SocketAddr) -> WsConn {
        // Now, split the socket into a reader and a writer
        let (sender, receiver) = socket.split();

        WsConn {
            connection_id: Uuid::new_v4(),
            room,
            lobby_addr: lobby,
            sender,
            receiver: Some(receiver),
            reader_task: None,
            who,
        }
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

        async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(s) => addr.do_send(RelayMessageToLobby(s.to_string())),
                    Message::Binary(b) => {
                        addr.do_send(RelayMessageToLobby(String::from_utf8(b).unwrap()));
                    }
                    Message::Close(_) => {
                        println!("Client {who} disconnected from websocket");

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
        .into_actor(self)
        .spawn(ctx);

        // self.reader_task = Some(reader_task);
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

impl Handler<WsCloseConnection> for WsConn {
    type Result = ();

    fn handle(&mut self, _msg: WsCloseConnection, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();

        // also send close message to the client
        println!("Sending close to {}...", self.who);

        let _x = self.sender.send(Message::Close(Some(CloseFrame {
            code: axum::extract::ws::close_code::NORMAL,
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
        println!("Received message from client: {}", self.who);

        // tell the lobby to send it to everyone else
        self.lobby_addr.do_send(ClientActorMessage {
            id: self.connection_id,
            msg: msg.0,
            room_id: self.room,
        });
    }
}
