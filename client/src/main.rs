mod music_actor;

use std::thread;
use actix::prelude::*;
use music_actor::MusicActor;
use music_actor::MusicMessage;
use std::time::Duration;

#[actix_rt::main]
async fn main() {
    let mut ma = MusicActor::new();
    let music_actor = ma.start();
    println!("Music actor is created.");

    music_actor
        .do_send(MusicMessage::Happy(Duration::from_secs(10)));
    println!("Happy music is playing.");

    thread::sleep(Duration::from_millis(1000));

    music_actor
        .send(MusicMessage::Sad(Duration::from_secs(8)))
        .await
        .unwrap();
    println!("Sad music is playing.");
    music_actor
        .send(MusicMessage::Angry(Duration::from_secs(15)))
        .await
        .unwrap();
    println!("Angry music is playing.");

}
