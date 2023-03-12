use serde::{Serialize, Deserialize};

use crate::patternslot::PatternSlot;
use crate::instrument::Instrument;

#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleFlag {
    LinearFrequencies,
    AmigaFrequencies,
}

pub type Row = Vec<PatternSlot>;
pub type Pattern = Vec<Row>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub flags: ModuleFlag,
    pub restart_position: u16,
    pub default_tempo: u16,
    pub default_bpm: u16,    
    pub pattern_order: Vec<u8>, // Defines the exact order for the XM patterns playback.
    pub pattern: Vec<Pattern>,
    pub instrument: Vec<Instrument>, // Instrument 1 has index 0, instrument 2 has index 1, etc.
}

impl Default for Module {
    fn default() -> Self {
        Module {
            name: "".to_string(),
            flags: ModuleFlag::LinearFrequencies,
            restart_position: 0,
            default_tempo: 6,
            default_bpm: 125,
            pattern_order: vec![],
            pattern: vec![],
            instrument: vec![],
        }
    }
}
