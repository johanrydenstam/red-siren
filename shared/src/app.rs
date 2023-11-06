use serde::{Deserialize, Serialize};
use anyhow::Result;
pub use crux_core::App;
use crux_core::render::Render;
use crux_kv::KeyValue;
use crux_macros::Effect;

mod instrument;
mod tuner;
mod intro;

pub use instrument::Instrument;
pub use tuner::Tuner;
pub use intro::Intro;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Activity {
    Intro,
    Tune,
    Play,
    Listen
}

impl Default for Activity {
    fn default() -> Self {
        Activity::Intro
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    instrument: instrument::Model,
    tuning: tuner::Model,
    intro: intro::Model,
    activity: Activity
}

#[derive(Serialize, Deserialize, Default)]
pub struct ViewModel {
    activity: Activity
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    None,
}

#[derive(Default)]
pub struct RedSiren {
    tuner: Tuner,
    instrument: Instrument,
    intro: Intro,
}

#[cfg_attr(feature = "typegen", derive(crux_macros::Export))]
#[derive(Effect)]
#[effect(app = "RedSiren")]
pub struct RedSirenCapabilities {
    pub render: Render<Event>,
    pub key_value: KeyValue<Event>,
}

impl App for RedSiren {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = RedSirenCapabilities;

    fn update(&self, msg: Event, model: &mut Model, caps: &RedSirenCapabilities) {
        match msg {
            Event::None => {}
        }

        caps.render.render();
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel { activity: model.activity }
    }
}

#[cfg(test)]
mod tests {}
