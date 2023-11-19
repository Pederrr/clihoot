mod dummy_client_actor;
mod server;
mod websocket;

use crate::dummy_client_actor::DummyClientActor;
use crate::websocket::{Subscribe, WebsocketActor};
use actix::prelude::*;
use actix::{Actor, Message};
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let sys = actix::System::new();

    // start a server on separated thread
    thread::spawn(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { server::dummy_echo_server::spawn_server().await.unwrap() })
    });
    thread::sleep(Duration::from_millis(1000));

    sys.block_on(async {
        // start websocket actor
        let addr_websocket_actor = WebsocketActor::new().await.start();

        // start our dummy client actor
        let addr_dummy = DummyClientActor {
            websocket_astor_addr: addr_websocket_actor.clone(),
        }
        .start();

        // lets set our dummy actor as a subscriber for incoming messages from server
        addr_websocket_actor
            .send(Subscribe(addr_dummy.recipient()))
            .await
            .unwrap();
    });
    sys.run()?;

    Ok(())
}
