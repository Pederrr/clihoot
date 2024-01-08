use actix::prelude::*;
use log::debug;
use ratatui::style::Color;
use ratatui::widgets::ListState;

use uuid::Uuid;

use crate::music_actor::{MusicActor, MusicMessage};
use crate::websocket::WebsocketActor;
use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};

use common::terminal::highlight::Theme;

use common::terminal::terminal_actor::{TerminalActor, TerminalStop};
use common::terminal::widgets::choice::{ChoiceGrid, ChoiceSelectorState};

#[derive(Debug)]
pub enum StudentTerminalState {
    StartGame,
    NameSelection {
        name: String,
        name_already_used: bool,
    },
    ColorSelection {
        list_state: ListState,
    },
    WaitingForGame {
        list_state: ListState,
    },
    Question {
        question: NextQuestion,
        players_answered_count: usize,
        answered: bool,
        start_time: chrono::DateTime<chrono::Utc>,
        duration_from_start: chrono::Duration,
        choice_grid: ChoiceGrid,
        choice_selector_state: ChoiceSelectorState,
    },
    Answers {
        answers: QuestionEnded,
    },
    Results {
        results: ShowLeaderboard,
        list_state: ListState,
    },
    EndGame, // show some screen saying that the game ended and the student should just pres ctrl + c to close the app
    Error {
        message: String,
    },
}

pub struct StudentTerminal {
    pub uuid: Uuid,
    pub name: String,
    pub color: Color,
    pub quiz_name: String,
    pub syntax_theme: Theme,
    pub players: Vec<PlayerData>,
    pub ws_actor_address: Addr<WebsocketActor>,
    pub state: StudentTerminalState,
    pub music_address: Addr<MusicActor>,
}

impl StudentTerminal {
    #[must_use]
    pub fn new(
        uuid: Uuid,
        quiz_name: String,
        ws_addr: Addr<WebsocketActor>,
        music_address: Addr<MusicActor>,
        syntax_theme: Theme,
    ) -> Self {
        Self {
            uuid,
            name: String::new(),
            color: Color::default(),
            quiz_name,
            players: Vec::new(),
            ws_actor_address: ws_addr,
            state: StudentTerminalState::StartGame,
            music_address,
            syntax_theme,
        }
    }
}

impl TerminalStop for StudentTerminal {
    fn stop(&mut self) -> anyhow::Result<()> {
        debug!("Stopping terminal actor for student");
        System::current().stop(); // we don't have to save or clean anything on the client side
        Ok(())
    }
}

pub async fn run_student(
    uuid: Uuid,
    quiz_name: String,
    ws_actor_addr: Addr<WebsocketActor>,
    music_actor_addr: Addr<MusicActor>,
    syntax_theme: Theme,
) -> anyhow::Result<Addr<TerminalActor<StudentTerminal>>> {
    let term = TerminalActor::new(StudentTerminal::new(
        uuid,
        quiz_name,
        ws_actor_addr,
        music_actor_addr.clone(),
        syntax_theme,
    ))
    .start();

    music_actor_addr.do_send(MusicMessage::Lobby);

    Ok(term)
}
