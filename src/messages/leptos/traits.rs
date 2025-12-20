use crossbeam_channel::{SendError, Sender};
use leptos::prelude::*;

pub trait LeptosChannelMessageSender {
    type Message: Send + Sync + 'static;

    fn tx(&self) -> StoredValue<Sender<Self::Message>>;

    #[inline]
    fn send(&self, message: Self::Message) -> Result<(), SendError<Self::Message>> {
        self.tx().with_value(|tx| tx.send(message))
    }
}
