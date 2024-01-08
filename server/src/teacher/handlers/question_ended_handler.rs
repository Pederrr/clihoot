use anyhow::bail;
use common::{
    messages::network::QuestionEnded,
    terminal::{terminal_actor::TerminalHandleQuestionEnded, widgets::choice::ChoiceGrid},
};
use log::debug;
use ratatui::widgets::ListState;

use crate::teacher::terminal::{TeacherTerminal, TeacherTerminalState};

impl TerminalHandleQuestionEnded for TeacherTerminal {
    fn handle_question_ended(&mut self, question_ended: QuestionEnded) -> anyhow::Result<()> {
        debug!("Teacher: handling question ended");

        let question = match &self.state {
            TeacherTerminalState::Question {
                question,
                players_answered_count: _,
                start_time: _,
                duration_from_start: _,
            } => question,
            _ => bail!(
                "Teacher: received question ended, but the terminal is not in the Question state"
            ),
        };

        if question.question_index != question_ended.question_index {
            bail!(
                "Teacher: received question ended, but the question index does not match the current question"
            );
        }

        self.state = TeacherTerminalState::Answers {
            answers: question_ended,
            list_state: ListState::default(),
            choice_grid: ChoiceGrid::default(),
        };

        Ok(())
    }
}
