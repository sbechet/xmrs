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

/// Vibrato with Steroid
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vibrato {
    pub waveform: Waveform,
    pub speed: u8, // 0x00..0x3F
    pub depth: u8, // 0x00..0x0F
    pub sweep: u8, // 0x00..0xFF (In other trackers may be 0..FFFF !)
}
