use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::backends::MsgBackend;
use crate::{Interrupted, Terminator};

use super::{action::Action, Chat, ConversationList, State};

pub struct StateStore {
    state_tx: UnboundedSender<State>,
}

impl StateStore {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<State>();

        (StateStore { state_tx }, state_rx)
    }

    pub async fn main_loop(
        self,
        mut terminator: Terminator,
        mut backend: impl MsgBackend,
        mut action_rx: UnboundedReceiver<Action>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        let conversations = backend.get_recent_contacts();
        let msgs = backend.get_messages(&conversations[0], Some(100));

        let mut state = State::new(
            Chat::new(conversations[0].clone(), msgs),
            ConversationList::new(conversations),
        );

        self.state_tx.send(state.clone())?;

        let result = loop {
            tokio::select! {
                // Handle any actions that are received
                Some(action) = action_rx.recv() => match action {
                    Action::Exit => {
                        let _ = terminator.terminate(Interrupted::UserInt);
                        break Interrupted::UserInt;
                    }
                    Action::SendMessage(msg) => {
                        backend.send_message(
                            msg
                        );
                    }
                    Action::FocusConversation(contact) => {
                        state.chat.contact = contact;
                    }
                    //_ => (),
                },


                // Handle Interruptions
                Ok(interrupted) = interrupt_rx.recv() => {
                    break interrupted;
                }
            }

            // Update state from backend
            state.chat.messages = backend.get_messages(&state.chat.contact, Some(100));
            state.conversations = ConversationList::new(backend.get_recent_contacts());

            // Send state out
            self.state_tx.send(state.clone())?;
        };

        Ok(result)
    }
}
