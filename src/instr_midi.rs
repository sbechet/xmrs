use serde::{Deserialize, Serialize};

/// Midi Instrument
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct InstrMidi {
    pub on: bool,
    pub channel: u8,
    pub program: u16,
    pub bend: u16,
}
