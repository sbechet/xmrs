use serde::{Deserialize, Serialize};

/// Envelope Point, frame for the abscissa, value for the ordinate
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct EnvelopePoint {
    /// Frame number of the point (X-coordinate)
    pub frame: u16,
    /// Value of the point (Y-coordinate)
    pub value: u16,
}

/// Envelope with Steroid
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
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
