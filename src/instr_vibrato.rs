use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

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
            Waveform::Sine => -(std::f32::consts::TAU * step).sin(),
            Waveform::Square => {
                /*
                 * -_
                 */
                if step < 0.5 {
                    -1.0
                } else {
                    1.0
                }
            },
            Waveform::RampUp => {
                /*
                 * /
                 *  \
                 */
                if step < 0.5 {
                    2.0 * step
                } else {
                    2.0 * step - 2.0
                }
            },
            Waveform::RampDown => {
                /*
                 *  \
                 * \
                 */
                if step < 0.5 {
                    - 2.0 * step
                } else {
                    2.0 - 2.0 * step
                }
            },

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
