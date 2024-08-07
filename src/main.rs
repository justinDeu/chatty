mod state;
mod termination;
mod ui;

use state::StateStore;
use termination::{create_termination, Interrupted, Terminator};
use ui::UiManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (terminator, mut interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::new();
    let (ui_manager, action_rx) = UiManager::new();

    tokio::try_join!(
        state_store.main_loop(terminator, action_rx, interrupt_rx.resubscribe()),
        ui_manager.main_loop(state_rx, interrupt_rx.resubscribe()),
    )?;
    Ok(())
}
