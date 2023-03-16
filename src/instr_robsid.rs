use serde::{Serialize, Deserialize};

use crate::instr_sid::InstrSid;

/// Rob Hubbard Effects
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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
    pub arpeggio_reset_mask: u8,    // if version=15 { 12=>-value } else { odd=>+12, even=>0 }
}


impl Default for RobEffects {
    fn default() -> Self {
        Self {
            vibrato: false,
            vibrato_depth: 0,
            vibrato_div: 0,
            pw: false,
            pw_speed: 0,
            pw_delay: 0,
            drum: false,
            skydive: false,
            skydive_config_if: 0,
            skydive_config_add: 0,
            arpeggio: false,
            arpeggio_reset_mask: 0
        }
    }
}

/// Rob Hubbard Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrRobSid {
    pub sid: InstrSid,
    pub fx: [RobEffects; 3],
}

impl Default for InstrRobSid {
    fn default() -> Self {
        Self {
            sid: InstrSid::default(),
            fx: [RobEffects::default(); 3],
        }
    }
}
