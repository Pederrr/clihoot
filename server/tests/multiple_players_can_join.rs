mod fixtures;
mod mocks;
mod utils;

use std::thread::JoinHandle;

use actix::Addr;

use rstest::rstest;
use server::{
    messages::teacher_messages::{ServerHardStop, TeacherHardStop},
    server::state::Lobby,
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState,
};

#[rstest]
#[tokio::test]
async fn multiple_players_can_join(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let players_count = 20;
    let mut players = Vec::with_capacity(players_count);

    for _i in 0..players_count {
        players.push(utils::join_new_player());
    }

    // wait for all promises at once
    let players = futures_util::future::join_all(players).await;

    // map the results to the player data
    let players = players
        .into_iter()
        .map(|res| res.unwrap().2)
        .collect::<Vec<_>>();

    // Get server state and assert that both are there
    let state = server.send(GetServerState).await?;

    assert!(state.waiting_players.is_empty());
    assert_eq!(state.joined_players.len(), players_count);

    // assert that all players are in the joined players
    for player in players {
        assert!(state.joined_players.contains_key(&player.uuid));
    }

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}