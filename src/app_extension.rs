use crate::prelude::{BevyQueryDuplex, QueryDataOwned};
use crate::systems::*;
use crate::traits::{HasReceiver, HasSender};
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

pub trait LeptosBevyApp {
    fn import_message_from_leptos<R, M>(&mut self, bevy_rx: R) -> &mut Self
    where
        M: Message,
        R: HasReceiver<M> + Resource;

    fn export_message_to_leptos<S, M>(&mut self, bevy_tx: S) -> &mut Self
    where
        M: Message + Clone,
        S: HasSender<M> + Resource;

    fn add_duplex_leptos_message<D, M>(&mut self, bevy_duplex: D) -> &mut Self
    where
        M: Message + Clone,
        D: HasReceiver<M> + HasSender<M> + Resource;

    fn sync_leptos_signal_with_resource<D, R>(&mut self, bevy_duplex: D) -> &mut Self
    where
        R: Resource + Clone,
        D: HasReceiver<R> + HasSender<R> + Resource;

    #[cfg(feature = "bevy_state")]
    fn sync_leptos_signal_with_state<D, S>(&mut self, bevy_duplex: D) -> &mut Self
    where
        S: bevy::state::state::FreelyMutableState + Clone,
        D: HasReceiver<S> + HasSender<S> + Resource;

    fn sync_leptos_signal_with_query<D, F>(&mut self, duplex: BevyQueryDuplex<D, F>) -> &mut Self
    where
        for<'a> D: QueryDataOwned<'a> + Send + Sync + 'static,
        F: QueryFilter + 'static;
}

impl LeptosBevyApp for App {
    fn import_message_from_leptos<R, M>(&mut self, bevy_rx: R) -> &mut Self
    where
        M: Message,
        R: HasReceiver<M> + Resource,
    {
        self.insert_resource(bevy_rx)
            .add_message::<M>()
            .init_resource::<ImportedMessageIds<M>>()
            .add_systems(
                PreUpdate,
                import_and_send_leptos_messages::<R, M>.in_set(ImportLeptosMessageSet),
            )
    }

    fn export_message_to_leptos<R, M>(&mut self, bevy_tx: R) -> &mut Self
    where
        M: Message + Clone,
        R: HasSender<M> + Resource,
    {
        self.insert_resource(bevy_tx)
            .add_message::<M>()
            .init_resource::<ImportedMessageIds<M>>()
            .add_systems(
                PostUpdate,
                read_and_export_leptos_messages::<R, M>.in_set(ExportLeptosMessageSet),
            )
    }

    fn add_duplex_leptos_message<D, M>(&mut self, bevy_duplex: D) -> &mut Self
    where
        M: Message + Clone,
        D: HasReceiver<M> + HasSender<M> + Resource,
    {
        self.insert_resource(bevy_duplex)
            .add_message::<M>()
            .add_systems(
                PreUpdate,
                import_and_send_leptos_messages::<D, M>.in_set(ImportLeptosMessageSet),
            )
            .add_systems(
                PostUpdate,
                read_and_export_leptos_messages::<D, M>.in_set(ExportLeptosMessageSet),
            )
    }

    fn sync_leptos_signal_with_resource<D, R>(&mut self, bevy_duplex: D) -> &mut Self
    where
        R: Resource + Clone,
        D: HasReceiver<R> + HasSender<R> + Resource,
    {
        for message in bevy_duplex.rx().try_iter() {
            self.insert_resource(message);
        }

        self.insert_resource(bevy_duplex).add_systems(
            Update,
            sync_signal_resource::<D, R>.in_set(SyncSignalResourceSet),
        )
    }

    #[cfg(feature = "bevy_state")]
    fn sync_leptos_signal_with_state<D, S>(&mut self, bevy_duplex: D) -> &mut Self
    where
        S: bevy::state::state::FreelyMutableState + Clone,
        D: HasReceiver<S> + HasSender<S> + Resource,
    {
        for message in bevy_duplex.rx().try_iter() {
            self.insert_state(message);
        }

        self.insert_resource(bevy_duplex)
            .add_systems(Update, sync_signal_state::<D, S>.in_set(SyncSignalStateSet))
    }

    fn sync_leptos_signal_with_query<D, F>(&mut self, duplex: BevyQueryDuplex<D, F>) -> &mut Self
    where
        for<'a> D: QueryDataOwned<'a> + Send + Sync + 'static,
        F: QueryFilter + 'static,
    {
        self.insert_resource(duplex.duplex)
            .add_systems(Update, sync_query::<D, F>.in_set(SyncQuerySet))
    }
}
