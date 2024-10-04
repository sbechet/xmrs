use serde::{Deserialize, Serialize};

use crate::instr_sid::InstrSid;

/// Rob Hubbard Effects
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct RobEffects {
    // 1. Vibrato
    pub vibrato: bool,
    pub vibrato_depth: u8,
    pub vibrato_div: u8,

    // 2. Pulse Width
    pub pw: bool,
    pub pw_speed: i8,
    pub pw_delay: u16,

    // 3. Drums
    pub drum: bool,

    // 4. Skydive
    pub skydive: bool,
    pub skydive_config_if: u8,
    pub skydive_config_add: u8,

    // 5. Arpeggio
    pub arpeggio: bool,
    pub arpeggio_reset_mask: u8, // if version=15 { 12=>-value } else { odd=>+12, even=>0 }
}

/// Rob Hubbard Generalized Instrument
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct InstrRobSid {
    pub sid: InstrSid,
    pub fx: [RobEffects; 3],
}
