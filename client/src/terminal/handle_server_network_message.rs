use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::messages::ServerNetworkMessage;
use common::terminal::terminal_actor::TerminalHandleServerNetworkMessage;

impl TerminalHandleServerNetworkMessage for StudentTerminal {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()> {
        match network_message {
            ServerNetworkMessage::JoinResponse(join) => {
                self.state = StudentTerminalState::WaitingForGame {
                    players: join.players,
                };
                Ok(())
            }
            ServerNetworkMessage::NextQuestion(question) => {
                self.state = StudentTerminalState::Question {
                    question,
                    players_answered_count: 0,
                    answered: false,
                };
                Ok(())
            }
            ServerNetworkMessage::QuestionUpdate(update) => {
                let StudentTerminalState::Question {
                    question,
                    players_answered_count,
                    answered: _,
                } = &mut self.state
                else {
                    anyhow::bail!("");
                };

                if question.question_index != update.question_index {
                    anyhow::bail!("bar");
                }

                *players_answered_count = update.players_answered_count;

                Ok(())
            }
            ServerNetworkMessage::QuestionEnded(question) => {
                self.state = StudentTerminalState::Answers { answers: question };
                Ok(())
            }
            ServerNetworkMessage::ShowLeaderboard(leaderboard) => {
                self.state = StudentTerminalState::Results {
                    results: leaderboard,
                };
                Ok(())
            }
            ServerNetworkMessage::PlayersUpdate(update) => {
                if let StudentTerminalState::WaitingForGame { players } = &mut self.state {
                    *players = update.players;
                }
                Ok(())
            }
            ServerNetworkMessage::TeacherDisconnected(_) => {
                self.state = StudentTerminalState::Error {
                    message: "Teacher disconnected from the game".to_string(),
                };
                Ok(())
            }
            _ => Ok(()),
        }
    }
}