use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::Context;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver},
};
use tokio_stream::StreamExt;

use super::panes::AppRouter;

// Why did I have to use this here to get Component in scope for AppRouter::new?
use crate::ui::components::Component;
use crate::ui::components::ComponentRender;
use crate::{
    state::{action::Action, State},
    Interrupted,
};

const RENDERING_TICK_RATE: Duration = Duration::from_millis(250);

pub struct UiManager {
    action_tx: mpsc::UnboundedSender<Action>,
}

impl UiManager {
    pub fn new() -> (Self, UnboundedReceiver<Action>) {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        (Self { action_tx }, action_rx)
    }

    pub async fn main_loop(
        self,
        mut state_rx: UnboundedReceiver<State>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        let mut router = {
            let state = state_rx.recv().await.unwrap();
            AppRouter::new(&state, self.action_tx.clone())
        };

        let mut terminal = setup_terminal()?;
        let mut ticker = tokio::time::interval(RENDERING_TICK_RATE);
        let mut crossterm_events = EventStream::new();

        let result: anyhow::Result<Interrupted> = loop {
            tokio::select! {
                _ = ticker.tick() => (),
                maybe_event = crossterm_events.next() => match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        router.handle_key_event(key);
                    },
                    None => break Ok(Interrupted::UserInt),
                    _ => (),
                },
                Some(state) = state_rx.recv() => {
                    println!("Received state");
                },
                Ok(interrupted) = interrupt_rx.recv() => {
                    break Ok(interrupted);
                }
            }

            if let Err(err) = terminal
                .draw(|frame| router.render(frame, ()))
                .context("could not render to the terminal")
            {
                break Err(err);
            }
        };

        // Restore the terminal to its original state
        restore_terminal(&mut terminal)?;
        result
    }
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(terminal.show_cursor()?)
}
