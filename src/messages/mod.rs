mod bevy;
mod leptos;

pub use crate::messages::bevy::*;
pub use crate::messages::leptos::*;
use crate::utils::init_rw_signal_from_receiver;

pub fn message_l2b<M>() -> (LeptosMessageSender<M>, BevyMessageReceiver<M>)
where
    M: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    (LeptosMessageSender::new(tx), BevyMessageReceiver::new(rx))
}

pub fn message_b2l<M>() -> (LeptosMessageReceiver<M>, BevyMessageSender<M>)
where
    M: Send + Sync + 'static,
{
    let (tx, rx) = crossbeam_channel::bounded(50);

    let signal = init_rw_signal_from_receiver(&rx);

    (
        LeptosMessageReceiver::new(rx, signal),
        BevyMessageSender::new(tx),
    )
}

pub fn message_duplex<M>() -> (LeptosMessageDuplex<M>, BevyMessageDuplex<M>)
where
    M: Send + Sync + 'static,
{
    let (tx_l2b, rx_l2b) = crossbeam_channel::bounded(50);
    let (tx_b2l, rx_b2l) = crossbeam_channel::bounded(50);

    let signal = init_rw_signal_from_receiver(&rx_b2l);

    (
        LeptosMessageDuplex::new(rx_b2l, signal, tx_l2b),
        BevyMessageDuplex::new(rx_l2b, tx_b2l),
    )
}
