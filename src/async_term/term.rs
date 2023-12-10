use actix::prelude::*;
use crossterm::event::KeyCode;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io::stdout;

pub enum TerminalState {
    NameSelection { name: String },
    ColorSelection { list_state: ListState },
    Todo,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct TerminalActor {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    name: String,
    color: Color,
    state: TerminalState,
}

const COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

impl TerminalActor {
    pub fn new() -> Self {
        let term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        Self {
            terminal: term,
            name: String::new(),
            color: Color::Red,
            state: TerminalState::NameSelection {
                name: String::new(),
            },
        }
    }

    pub fn redraw(&mut self) -> anyhow::Result<()> {
        // TODO the function for drawing different for each "screen"
        //      eg. enum and then using drawing function according to the enum member

        // screens:
        //  - choosing a name / color for avatar
        //  - waiting in lobby to start game
        //  - having a question with answers on the screen
        //    (also with the count of people that answered + COUNTDOWN)
        //  - being in the lobby waiting for others to answer
        //  - showing the results of the question
        //  - showing the final results of the game

        // teacher screens:
        //   - TODO
        match &mut self.state {
            TerminalState::NameSelection { name } => {
                self.terminal.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new(format!("Name: {}|", name)).block(
                            Block::default()
                                .title("Write your name")
                                .borders(Borders::ALL),
                        ),
                        frame.size(),
                    );
                })?;
            }
            TerminalState::ColorSelection { list_state } => {
                self.terminal.draw(|frame| {
                    let default_block = Block::default()
                        .title("Select your color")
                        .borders(Borders::ALL);

                    // TOOD constant for this
                    let items: Vec<_> = COLORS
                        .iter()
                        .map(|color| ListItem::new(format!("{:?}", color)))
                        .collect();

                    frame.render_stateful_widget(
                        List::new(items)
                            .block(default_block)
                            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                            .highlight_symbol(">> "),
                        frame.size(),
                        list_state,
                    );
                })?;
            }
            TerminalState::Todo => {
                self.terminal.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new(format!(
                            "Your name is \"{}\" and your color is \"{:?}\".",
                            self.name, self.color
                        ))
                        .block(Block::default().title("Greeting").borders(Borders::ALL)),
                        frame.size(),
                    );
                })?;
            }
        }
        Ok(())
    }
}

impl Actor for TerminalActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
    }
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Stop;

impl Handler<Stop> for TerminalActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Stop, ctx: &mut Context<Self>) -> Self::Result {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        ctx.stop();
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Redraw;

impl Handler<Redraw> for TerminalActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Redraw, _ctx: &mut Self::Context) -> Self::Result {
        self.redraw()
    }
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct KeyPress {
    pub key_code: KeyCode,
}

impl Handler<KeyPress> for TerminalActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: KeyPress, _ctx: &mut Self::Context) -> Self::Result {
        // TODO define these as functions for every state
        match &mut self.state {
            TerminalState::NameSelection { name } => match msg.key_code {
                KeyCode::Backspace => {
                    name.pop();
                }
                KeyCode::Char(char) => {
                    name.push(char);
                }
                KeyCode::Enter => {
                    self.name = name.to_string();
                    self.state = TerminalState::ColorSelection {
                        list_state: ListState::default().with_selected(Some(0)),
                    }
                }
                _ => {}
            },
            TerminalState::ColorSelection { list_state } => {
                let mut selected = list_state.selected().unwrap_or(0);

                match msg.key_code {
                    KeyCode::Backspace => {
                        self.state = TerminalState::NameSelection {
                            name: self.name.to_string(),
                        };
                    }
                    KeyCode::Enter => {
                        self.color = COLORS[list_state.selected().unwrap_or(0)];
                        self.state = TerminalState::Todo;
                    }
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                        selected += 1;
                        if selected >= 3 {
                            selected = 0;
                        }
                        list_state.select(Some(selected))
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                        if selected == 0 {
                            selected = 2;
                        } else {
                            selected -= 1;
                        }
                        list_state.select(Some(selected))
                    }
                    _ => {}
                };
            }
            TerminalState::Todo => {}
        };
        self.redraw()
    }
}
