use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct EnvelopePoint {
    /// Frame number of the point (X-coordinate)
    pub frame: u16,
    /// Value of the point (Y-coordinate)
    pub value: u16,
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
    /// 12 points maximum for XM compatibility
    pub point: Vec<EnvelopePoint>,

    pub sustain_enabled: bool,
    /// index in `point`
    pub sustain_point: u8,

    pub loop_enabled: bool,
    /// index in `point`
    pub loop_start_point: u8,
    /// index in `point`
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
