use actix::prelude::*;
use actix::Addr;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use std::sync::Arc;

mod term;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let term = Arc::new(term::TerminalActor::new().start());

    term.send(term::Redraw).await??;

    tokio::spawn(handle_input(term)).await??;

    Ok(())
}

async fn handle_input(term: Arc<Addr<term::TerminalActor>>) -> anyhow::Result<()> {
    let mut reader = crossterm::event::EventStream::new();

    loop {
        let crossterm_event = reader.next().fuse();
        tokio::select! {
            maybe_event = crossterm_event => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        if key.kind == KeyEventKind::Press {
                            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                                term.send(term::Stop).await??;
                                break;
                            }

                            term.send(term::KeyPress {key_code: key.code}).await??;
                        }
                    }
                    Some(Err(e)) => return Err(e.into()),
                    None => {}
                    _ => todo!() // screen resize
                }
            }
        }
    }

    Ok(())
}
