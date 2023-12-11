use actix::prelude::Message;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketGracefulStop;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketHardStop;

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DisconnectFromLobby {
    pub player_id: Uuid,
}