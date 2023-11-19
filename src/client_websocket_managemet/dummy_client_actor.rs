use actix::prelude::*;
use actix::{Actor, Context};
use std::time::Duration;

use crate::websocket::{MessageFromServer, WebsocketActor, WebsocketMsg};

// actor used as a demonstration that actor can receive and send messages from and to the server
pub struct DummyClientActor {
    pub(crate) websocket_astor_addr: Addr<WebsocketActor>,
}

impl DummyClientActor {
    // sends a message to the server (this could be send from anywhere we have address to the websocket actor)
    fn send_message_to_server(&self) {
        self.websocket_astor_addr.do_send(WebsocketMsg(
            "C IS BETTER THEN RUST, CHANGE MY MIND".to_string(),
        ));
    }
}

impl Actor for DummyClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Dummy client actor is alive");

        // lets send a message to the server every 3 seconds
        ctx.run_interval(Duration::from_secs(3), |act, ctx| {
            act.send_message_to_server();
        });
    }
}

impl Handler<MessageFromServer> for DummyClientActor {
    type Result = ();

    // handle messages from server
    fn handle(&mut self, msg: MessageFromServer, ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "Inside client actor handler: message from server arrive: {}",
            msg.content
        );
    }
}
