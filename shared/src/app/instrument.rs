use serde::{Deserialize, Serialize};

use crux_core::render::Render;
use crux_core::App;
use crux_kv::KeyValue;
use crux_macros::Effect;


#[derive(Default)]
pub struct Instrument;


#[derive(Default, Serialize, Deserialize)]
pub struct Model {

}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewModel {

}

pub enum Event {
  None
}

#[cfg_attr(feature = "typegen", derive(crux_macros::Export))]
#[derive(Effect)]
#[effect(app = "Instrument")]
pub struct InstrumentCapabilities {
    pub render: Render<Event>,
    pub key_value: KeyValue<Event>,
}

impl App for Instrument {
    type Event = Event;

    type Model = Model;

    type ViewModel = ViewModel;

    type Capabilities = InstrumentCapabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        todo!()
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        todo!()
    }
}