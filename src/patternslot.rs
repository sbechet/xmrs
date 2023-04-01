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
