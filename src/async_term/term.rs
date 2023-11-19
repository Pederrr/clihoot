use actix::prelude::*;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::stdout;

pub struct TerminalActor {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    counter: usize,
}

impl TerminalActor {
    pub fn new() -> Self {
        let term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        Self {
            terminal: term,
            counter: 0,
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
        self.terminal.draw(|frame| {
            frame.render_widget(
                Paragraph::new(format!("{}", self.counter))
                    .block(Block::default().title("Greeting").borders(Borders::ALL)),
                frame.size(),
            );
        })?;
        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Stop;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Redraw;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Increment;

impl Actor for TerminalActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // TODO this functions can fail
        enable_raw_mode();
        stdout().execute(EnterAlternateScreen);
    }
}

impl Handler<Stop> for TerminalActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: Stop, ctx: &mut Context<Self>) -> Self::Result {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        ctx.stop();
        Ok(())
    }
}

impl Handler<Increment> for TerminalActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: Increment, ctx: &mut Context<Self>) -> Self::Result {
        self.counter += 1;
        Ok(())
    }
}

impl Handler<Redraw> for TerminalActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: Redraw, ctx: &mut Self::Context) -> Self::Result {
        self.redraw()
    }
}
