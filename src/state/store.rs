use std::time::Duration;

use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::{Interrupted, Terminator};

use super::{action::Action, backends::MsgBackend, State};

// TODO: Need to create State and update sender type
pub struct StateStore<T: MsgBackend> {
    state_tx: UnboundedSender<State<T>>,
}

/*
 * Beyond just adding the normal trait bound stuff, I had to add the static lifetime. This is
 * necessary since the `main_loop` *must* have a known address, but with generics (and
 * subsequently, monomorphization) the compiler will place the function pointer literally anywhere
 */
impl<T: MsgBackend + 'static> StateStore<T> {
    pub fn new() -> (Self, UnboundedReceiver<State<T>>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<State<T>>();

        (StateStore { state_tx }, state_rx)
    }

    /*
     * Fun fact, it is not possible to implement the `Default` trait generically. Therefore, I
     * added the `state` to the function signature at the highest point (main.rs) where it can be
     * initialized using the `type Backend = ...` stuff
     */
    pub async fn main_loop(
        self,
        mut state: State<T>,
        mut terminator: Terminator,
        mut action_rx: UnboundedReceiver<Action>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        self.state_tx.send(state.clone())?;

        let mut ticker = tokio::time::interval(Duration::from_secs(1));

        let result = loop {
            tokio::select! {
                // Handle any actions that are received
                Some(action) = action_rx.recv() => match action {
                    Action::Exit => {
                        let _ = terminator.terminate(Interrupted::UserInt);
                        break Interrupted::UserInt;
                    },
                    Action::SendMessage(msg) => {
                        state.chat.send_msg(msg);
                    },
                    //_ => unreachable!(),
                },

                // Tick timer updating state
                _ = ticker.tick() => {
                    state.update();
                },

                // Handle Interruptions
                Ok(interrupted) = interrupt_rx.recv() => {
                    break interrupted;
                }
            }

            // Send state out
            self.state_tx.send(state.clone())?;
        };

        Ok(result)
    }
}
