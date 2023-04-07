use crate::note::Note;
use serde::{Deserialize, Serialize};

/// A typical pattern slot
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
#[repr(C)]
pub struct PatternSlot {
    pub note: Note,
    /// 0: none, 1-128
    pub instrument: u8,
    /// 0..64, 255
    pub volume: u8,
    pub effect_type: u8,
    pub effect_parameter: u8,
}

impl PatternSlot {
    pub fn has_tone_portamento(&self) -> bool {
        self.effect_type == 3 || self.effect_type == 5 || self.volume >> 4 == 0x0F
    }

    pub fn has_arpeggio(&self) -> bool {
        self.effect_type == 0 && self.effect_parameter != 0
    }

    pub fn has_vibrato(&self) -> bool {
        self.effect_type == 4 || self.effect_type == 6 || self.volume >> 4 == 0x0B
    }
}
