use actix::prelude::*;
use std::sync::Arc;

mod term;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let term = Arc::new(term::TerminalActor::new().start());

    // TODO actor/task/thread that would periodically send redraw message
    // TODO actor/task/thread that would read user input and notify the Terminal actor
    term.send(term::Redraw).await?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;
    term.send(term::Increment).await?;
    term.send(term::Redraw).await?;

    std::thread::sleep(std::time::Duration::from_secs(2));
    term.send(term::Stop).await?;

    Ok(())
}
