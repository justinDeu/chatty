mod state;
mod termination;
mod ui;

use state::{State, StateStore};
use termination::{create_termination, Interrupted, Terminator};
use ui::UiManager;

/*
 * This is a common pattern. Since setting a "default" generic value is not possible, you will
 * often see a type declared based on `cfg` values and then use that type as the "generic". I use
 * this for my neural network project using `burn`.
 */
type Backend = crate::state::backends::MockBackend;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (terminator, interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::<Backend>::new();
    let (ui_manager, action_rx) = UiManager::new();
    let state = State::<Backend>::default();

    tokio::try_join!(
        state_store.main_loop(state, terminator, action_rx, interrupt_rx.resubscribe()),
        ui_manager.main_loop(state_rx, interrupt_rx.resubscribe()),
    )?;
    Ok(())
}
