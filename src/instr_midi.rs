use serde::{Serialize, Deserialize};

/// Midi Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrMidi {
    pub on: bool,
    pub channel: u8,
    pub program: u16,
    pub bend: u16,
}

impl Default for InstrMidi {
    fn default() -> Self {
        Self {
            on: false,
            channel: 0,
            program: 0,
            bend: 0,
        }
    }
}
