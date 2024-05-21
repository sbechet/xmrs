use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Envelope Point, frame for the abscissa, value for the ordinate
#[derive(Default, bincode::Encode, Serialize, bincode::Decode, Deserialize, Copy, Clone, Debug)]
pub struct EnvelopePoint {
    /// Frame number of the point (X-coordinate)
    pub frame: u16,
    /// Value of the point (Y-coordinate)
    pub value: u16,
}

impl EnvelopePoint {
    /// Linear interpolation between two envelope points
    pub fn lerp(a: &EnvelopePoint, b: &EnvelopePoint, pos: u16) -> f32 {
        if pos <= a.frame {
            return a.value as f32;
        } else if pos >= b.frame {
            return b.value as f32;
        } else {
            let p: f32 = (pos - a.frame) as f32 / (b.frame - a.frame) as f32;
            return a.value as f32 * (1.0 - p) + b.value as f32 * p;
        }
    }
}

/// Envelope with Steroid
#[derive(Default, bincode::Encode, Serialize, bincode::Decode, Deserialize, Clone, Debug)]
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
