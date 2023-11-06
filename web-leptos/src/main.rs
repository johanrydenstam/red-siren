mod core;

use leptos::{component, create_effect, create_signal, view, IntoView, SignalGet, SignalUpdate};
use shared::Event;

#[component]
fn RootComponent() -> impl IntoView {
    // let core = core::new();
    // let (view, render) = create_signal(core.view());
    // let (event, set_event) = create_signal(Event::Reset);

    // create_effect(move |_| {
    //     core::update(&core, event.get(), render);
    // });

    view! {
        <div>{"hello siren"}</div>
    }
}

fn main() {
    leptos::mount_to_body(|| {
        view! { <RootComponent /> }
    });
}
