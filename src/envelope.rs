use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Envelope Point, frame for the abscissa, value for the ordinate
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct EnvelopePoint {
    /// Frame number of the point (X-coordinate)
    pub frame: usize,
    /// Value of the point (Y-coordinate)
    pub value: f32,
}

impl EnvelopePoint {
    /// Linear interpolation between two envelope points
    pub fn lerp(a: &EnvelopePoint, b: &EnvelopePoint, pos: usize) -> f32 {
        if pos <= a.frame {
            return a.value;
        } else if pos >= b.frame {
            return b.value;
        } else {
            let p: f32 = (pos - a.frame) as f32 / (b.frame - a.frame) as f32;
            return a.value * (1.0 - p) + b.value * p;
        }
    }
}

/// Envelope with Steroid
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Envelope {
    pub enabled: bool,
    /// 12 points maximum for XM compatibility
    pub point: Vec<EnvelopePoint>,

    pub sustain_enabled: bool,
    /// index in `point`
    pub sustain_point: usize,

    pub loop_enabled: bool,
    /// index in `point`
    pub loop_start_point: usize,
    /// index in `point`
    pub loop_end_point: usize,
}
