use bevy::prelude::*;
use leptos::prelude::*;

use crate::{
    messages::{message_l2b, LeptosChannelMessageSender},
    plugin::{LeptosBevyCanvasCleanup, LeptosBevyCanvasPlugin},
    prelude::LeptosBevyApp,
};

#[component]
pub fn BevyCanvas(
    init: impl FnOnce() -> App + 'static,
    #[prop(into, default = "bevy_canvas".to_string())]
    canvas_id: String,
) -> impl IntoView {
    let (shutdown_canvas, set_shutdown_canvas) = message_l2b::<LeptosBevyCanvasCleanup>();

    request_animation_frame(move || {
        let mut app = init();
        app.add_plugins(LeptosBevyCanvasPlugin)
            .import_message_from_leptos(set_shutdown_canvas);
        app.run();
    });

    on_cleanup(move || {
        shutdown_canvas
            .send(LeptosBevyCanvasCleanup)
            .expect("couldn't send cleanup to bevy app");
    });

    view! { <canvas id=canvas_id></canvas> }
}
