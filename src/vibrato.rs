use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Vibrato Waveform
#[derive(Default, Serialize, Deserialize, Clone, Copy, IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum Waveform {
    #[default]
    Sine = 0,
    RampDown = 1,
    Square = 2,
    Random = 3,
    RampUp = 4,
}

impl Waveform {
    /// Get waveform. Step in range 0..64
    pub fn waveform(&self, step: u16) -> f32 {
        let step = step % 64;
        return match &self {
            Waveform::Sine => -(std::f32::consts::TAU * step as f32 / 64.0).sin(),
            Waveform::RampDown => (32.0 - step as f32) / 32.0, // 1.0 when step = 0; -1.0 when step = 0x40
            Waveform::Square => {
                if step >= 32 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Random => rand::random::<f32>(),
            Waveform::RampUp => (step as f32 - 32.0) / 32.0,
        };
    }
}

/// Vibrato with Steroid
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vibrato {
    pub waveform: Waveform,
    pub speed: u8, // 0x00..0x3F
    pub depth: f32, // 0.0..1.0
    pub sweep: u8, // 0x00..0xFF (In other trackers may be 0..FFFF !)
}
