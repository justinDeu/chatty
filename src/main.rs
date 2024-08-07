mod state;
mod termination;

use state::StateStore;
use termination::{create_termination, Interrupted, Terminator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (terminator, mut interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::new();

    tokio::try_join!(state_store.main_loop(terminator, state_rx, interrupt_rx.resubscribe()),)?;
    Ok(())
}
