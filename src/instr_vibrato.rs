use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[cfg(feature = "micromath")]
use micromath::F32Ext;
#[cfg(feature = "libm")]
use num_traits::float::Float;

/// Vibrato Waveform
#[derive(Default, Serialize, Deserialize, Clone, Copy, IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum Waveform {
    #[default]
    Sine = 0,
    Square = 1,
    RampUp = 2,
    RampDown = 3,
}

impl Waveform {
    // instr autovib
    pub fn value(&self, step: f32) -> f32 {
        let step = step % 1.0;
        return match &self {
            Waveform::Sine => 0.5 + 0.5 * (core::f32::consts::TAU * (step + 0.25)).sin(),
            Waveform::Square => {
                if step < 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            Waveform::RampUp => {
                if step < 0.5 {
                    0.5 * step
                } else {
                    0.5 * step + 0.5
                }
            }
            Waveform::RampDown => {
                if step < 0.5 {
                    1.0 - 0.5 * step
                } else {
                    -0.5 * step + 0.5
                }
            }
        };
    }
}

/// Instrument Vibrato
#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct InstrVibrato {
    pub waveform: Waveform,
    pub speed: f32,
    pub depth: f32,
    pub sweep: f32,
}
