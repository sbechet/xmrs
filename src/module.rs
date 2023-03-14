use serde::{Serialize, Deserialize};

use crate::patternslot::PatternSlot;
use crate::instrument::Instrument;

#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleFlag {
    LinearFrequencies,
    AmigaFrequencies,
}

/// A row contains its column elements
pub type Row = Vec<PatternSlot>;

/// Patterns are sequences of lines
pub type Pattern = Vec<Row>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub flags: ModuleFlag,
    /// Restart index in `pattern_order`
    pub restart_position: u16,
    pub default_tempo: u16,
    pub default_bpm: u16,    
    /// Defines the exact order for the patterns playback
    pub pattern_order: Vec<u8>, 
    pub pattern: Vec<Pattern>,
    /// Instrument 1 has index 0, instrument 2 has index 1, etc.
    pub instrument: Vec<Instrument>, 
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
