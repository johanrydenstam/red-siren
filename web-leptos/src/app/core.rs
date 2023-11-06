use std::rc::Rc;

use leptos::{SignalUpdate, WriteSignal};
use shared::{RedSirenCapabilities, RedSiren, Effect, Event, ViewModel};

pub type Core = Rc<shared::Core<Effect, RedSiren>>;

pub fn new() -> Core {
    Rc::new(shared::Core::new::<RedSirenCapabilities>())
}

pub fn update(core: &Core, event: Event, render: WriteSignal<ViewModel>) {
    for effect in core.process_event(event) {
        process_effect(core, effect, render);
    }
}

pub fn process_effect(core: &Core, effect: Effect, render: WriteSignal<ViewModel>) {
    match effect {
        Effect::Render(_) => {
            render.update(|view| *view = core.view());
        }
        Effect::KeyValue(_) => {
            render.update(|view| *view = core.view());
        }
    };
}
