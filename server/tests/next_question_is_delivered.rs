mod fixtures;
mod mocks;
mod utils;

use std::thread::JoinHandle;

use actix::Addr;
use anyhow::bail;
use common::test_utils::compare_censored_questions;
use common::{
    assert_censored_question_eq, model::ServerNetworkMessage, questions::QuestionCensored,
};
use rstest::rstest;
use server::{
    messages::teacher_messages::{ServerHardStop, StartQuestionMessage, TeacherHardStop},
    server::state::{Lobby, Phase},
    teacher::init::Teacher,
};

use crate::{
    fixtures::create_server_and_teacher::create_server_and_teacher,
    mocks::get_server_state_handler::GetServerState, utils::sample_questions,
};

#[rstest]
#[tokio::test]
async fn next_question_is_delivered(
    create_server_and_teacher: (JoinHandle<()>, Addr<Lobby>, JoinHandle<()>, Addr<Teacher>),
) -> anyhow::Result<()> {
    let (server_thread, server, teacher_thread, teacher) = create_server_and_teacher;

    let (_sender, mut receiver, _data) = utils::join_new_player().await?;

    // The teacher now starts the question
    server.send(StartQuestionMessage).await??;

    // and the client should receive the question message
    let questions = sample_questions();

    let question = utils::receive_server_network_msg(&mut receiver).await?;
    let question = match question {
        ServerNetworkMessage::NextQuestion(q) => q,
        _ => bail!("Expected NextQuestion"),
    };

    assert_eq!(question.question_index, 0);
    assert_eq!(question.questions_count, questions.len());
    assert_eq!(
        question.show_choices_after,
        questions[0].get_reading_time_estimate()
    );

    assert_censored_question_eq!(
        &question.question,
        QuestionCensored::from(questions[0].clone())
    );

    let state = server.send(GetServerState).await?;
    assert_eq!(state.phase, Phase::ActiveQuestion(0));

    server.send(ServerHardStop).await?;
    server_thread.join().expect("Server thread panicked");

    teacher.send(TeacherHardStop).await?;
    teacher_thread.join().expect("Teacher thread panicked");

    Ok(())
}
