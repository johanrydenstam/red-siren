use crux_core::{render::Render, Capability};
pub use crux_core::App;
use crux_kv::KeyValue;
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

pub mod instrument;
pub mod intro;
pub mod tuner;

pub use instrument::Instrument;
pub use intro::Intro;
pub use tuner::Tuner;

use self::{intro::IntroCapabilities, tuner::TunerCapabilities, instrument::InstrumentCapabilities};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Activity {
    Intro,
    Tune,
    Play,
    Listen,
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
    activity: Activity,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ViewModel {
    pub activity: Activity,
    pub intro: intro::ViewModel,
    pub tuning: tuner::Model,
    pub instrument: instrument::ViewModel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Event {
    None,
    TunerEvent(tuner::Event),
    InstrumentEvent(instrument::Event),
    IntroEvent(intro::Event),
}

#[derive(Default)]
pub struct RedSiren {
    pub tuner: Tuner,
    pub instrument: Instrument,
    pub intro: Intro,
}

#[cfg_attr(feature = "typegen", derive(crux_macros::Export))]
#[derive(Effect)]
#[effect(app = "RedSiren")]
pub struct RedSirenCapabilities {
    pub render: Render<Event>,
    pub key_value: KeyValue<Event>,
}

impl From<&RedSirenCapabilities> for IntroCapabilities {
    fn from(incoming: &RedSirenCapabilities) -> Self {
        IntroCapabilities {
            key_value: incoming.key_value.map_event(super::Event::IntroEvent),
            render: incoming.render.map_event(super::Event::IntroEvent),
        }
    }
}

impl From<&RedSirenCapabilities> for TunerCapabilities {
    fn from(incoming: &RedSirenCapabilities) -> Self {
        TunerCapabilities {
            key_value: incoming.key_value.map_event(super::Event::TunerEvent),
            render: incoming.render.map_event(super::Event::TunerEvent),
        }
    }
}

impl From<&RedSirenCapabilities> for InstrumentCapabilities {
    fn from(incoming: &RedSirenCapabilities) -> Self {
        InstrumentCapabilities {
            key_value: incoming.key_value.map_event(super::Event::InstrumentEvent),
            render: incoming.render.map_event(super::Event::InstrumentEvent),
        }
    }
}

impl App for RedSiren {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = RedSirenCapabilities;

    fn update(&self, msg: Event, model: &mut Model, caps: &RedSirenCapabilities) {
        match msg {
            Event::None => {
                caps.render.render();
            }
            Event::InstrumentEvent(event) => {
                self.instrument
                    .update(event, &mut model.instrument, &caps.into())
            }
            Event::TunerEvent(event) => self.tuner.update(event, &mut model.tuning, &caps.into()),
            Event::IntroEvent(event) => self.intro.update(event, &mut model.intro, &caps.into()),
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        ViewModel {
            activity: model.activity,
            tuning: self.tuner.view(&model.tuning),
            intro: self.intro.view(&model.intro),
            instrument: self.instrument.view(&model.instrument)
        }
    }
}

#[cfg(test)]
mod tests {}
