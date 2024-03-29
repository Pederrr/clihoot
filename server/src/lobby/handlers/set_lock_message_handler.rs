use actix::{Context, Handler};

use crate::{messages::lobby::SetLockMessage, Lobby};

use log::debug;

impl Handler<SetLockMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: SetLockMessage, _: &mut Context<Self>) -> Self::Result {
        debug!(
            "Received SetLockMessage in Lobby; setting `locked` to `{}`",
            msg.locked
        );
        self.locked = msg.locked;
    }
}
