mod backends;
mod logging;
mod panic_handler;
mod state;
mod termination;
mod ui;

use core::panic;

use backends::MockBackend;
use logging::initialize_logging;
use panic_handler::initialize_panic_handler;
use state::StateStore;
use termination::{create_termination, Interrupted, Terminator};
use tracing::info;
use ui::UiManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_panic_handler()?;

    let _ = initialize_logging().map_err(|_| panic!("could not init logging"));

    info!("Beginning Chatty startup sequence");

    let (terminator, mut interrupt_rx) = create_termination();
    let (state_store, state_rx) = StateStore::new();
    let (ui_manager, action_rx) = UiManager::new();

    info!("Creating backend...");
    let backend = MockBackend::default();

    info!("Starting main loops...");
    tokio::try_join!(
        state_store.main_loop(terminator, backend, action_rx, interrupt_rx.resubscribe()),
        ui_manager.main_loop(state_rx, interrupt_rx.resubscribe()),
    )?;
    info!("Exiting...");
    Ok(())
}
