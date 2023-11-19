use actix::prelude::*;
use std::sync::Arc;

mod term;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let term = Arc::new(term::TerminalActor::new().start());

    // TODO find out how to receive text/other input from keyboard in order to select choices etc.
    // -- does not have to be an actor but e.g. a thread
    // -- if easier, can work with polling

    // TODO actor/task/thread that would read user input and notify the Terminal actor
    term.send(term::Redraw).await?;

    std::thread::sleep(std::time::Duration::from_millis(2));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));

    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;

    std::thread::sleep(std::time::Duration::from_millis(2));
    term.send(term::Stop).await?;

    Ok(())
}
