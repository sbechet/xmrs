use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct EnvelopePoint {
    pub frame: u16, // Frame number of the point (X-coordinate)
    pub value: u16, // Value of the point (Y-coordinate)
}

impl Default for EnvelopePoint {
    fn default() -> Self {
        Self {
            frame: 0,
            value: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Envelope {
    pub enabled: bool,

    pub point: Vec<EnvelopePoint>,

    pub sustain_enabled: bool,
    pub sustain_point: u8,

    pub loop_enabled: bool,
    pub loop_start_point: u8,
    pub loop_end_point: u8,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            enabled: false,

            point: vec![],

            sustain_enabled: false,
            sustain_point: 0,

            loop_enabled: false,
            loop_start_point: 0,
            loop_end_point: 0,
        }
    }
}
