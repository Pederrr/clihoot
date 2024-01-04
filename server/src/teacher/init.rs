use std::sync::mpsc::Sender;

use actix::{prelude::Actor, Addr};

use common::terminal::{
    handle_terminal_events::handle_events, messages::Initialize, terminal_actor::TerminalActor,
};

use crate::{lobby::state::Lobby, messages::lobby::RegisterTeacher};

use super::terminal::TeacherTerminal;

pub type Teacher = TerminalActor<TeacherTerminal>;

fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Could not create tokio runtime") // cannot seem to get rid of this
}

pub fn run_teacher(
    lobby: Addr<Lobby>,
    tx: Sender<Addr<Teacher>>,
    quiz_name: &str,
) -> anyhow::Result<()> {
    let system = actix::System::with_tokio_rt(create_tokio_runtime);

    system.block_on(init(lobby, tx, quiz_name))?;

    system.run()?;

    Ok(())
}

#[allow(clippy::unused_async)]
async fn init(
    lobby: Addr<Lobby>,
    tx: Sender<Addr<Teacher>>,
    quiz_name: &str,
) -> anyhow::Result<()> {
    let teacher =
        TerminalActor::new(TeacherTerminal::new(quiz_name.to_string(), lobby.clone())).start();

    tx.send(teacher.clone())
        .expect("Failed to send teacher address");

    lobby.do_send(RegisterTeacher {
        teacher: teacher.clone(),
    });

    // TODO: move the next 2 lines into the TerminalActor start method
    teacher.send(Initialize).await??;

    tokio::spawn(handle_events(teacher));

    // // handle CTRL+C gracefully
    // // TODO: Probably remove - since raw_mode does not send signals
    // tokio::spawn(async move {
    //     tokio::signal::ctrl_c()
    //         .await
    //         .expect("Failed to register CTRL-C handler");
    //     warn!("CTRL-C received, shutting down");
    //     System::current().stop();
    // });

    Ok(())
}
