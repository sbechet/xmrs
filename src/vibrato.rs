use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// Vibrato Waveform 
#[derive(Serialize, Deserialize, Clone, Copy, IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum Waveform {
    Sine = 0,
    RampDown = 1,
    Square = 2,
    Random = 3,
    RampUp = 4,
}

/// Vibrato with Steroid
#[derive(Serialize, Deserialize, Debug)]
pub struct Vibrato {
    pub waveform: Waveform,
    pub speed: u8, // 0x00..0x3F
    pub depth: u8, // 0x00..0x0F
    pub sweep: u8, // 0x00..0xFF (In other trackers may be 0..FFFF !)
}

impl Default for Vibrato {
    fn default() -> Self {
        Self {
            waveform: Waveform::Sine,
            speed: 0,
            depth: 0,
            sweep: 0,
        }
    }
}
