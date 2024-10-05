use serde::{Deserialize, Serialize};

use crate::instrument::Instrument;
use crate::patternslot::PatternSlot;
use crate::period_helper::FrequencyType;

use alloc::string::String;
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[cfg(target_pointer_width = "16")]
pub const MAX_NUM_ROWS: usize = 255;

#[cfg(target_pointer_width = "32")]
pub const MAX_NUM_ROWS: usize = 4095;

#[cfg(target_pointer_width = "64")]
pub const MAX_NUM_ROWS: usize = 4095;

/// A row contains its column elements
pub type Row = Vec<PatternSlot>;

/// Patterns are sequences of lines
pub type Pattern = Vec<Row>;

/// SoundTracker Module with Steroid
#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub comment: String,
    pub frequency_type: FrequencyType,
    /// Restart index in `pattern_order`
    pub restart_position: usize,
    pub default_tempo: u16,
    pub default_bpm: u16,
    /// Defines the exact order for the patterns playback
    pub pattern_order: Vec<usize>,
    pub pattern: Vec<Pattern>,
    /// Instrument 1 has index 0, instrument 2 has index 1, etc.
    pub instrument: Vec<Instrument>,
}

impl Default for Module {
    fn default() -> Self {
        Module {
            name: "".to_string(),
            comment: "".to_string(),
            frequency_type: FrequencyType::LinearFrequencies,
            restart_position: 0,
            default_tempo: 6,
            default_bpm: 125,
            pattern_order: vec![],
            pattern: vec![],
            instrument: vec![],
        }
    }
}

impl Module {
    /// get song length
    pub fn get_song_length(&self) -> usize {
        self.pattern_order.len()
    }

    /// get number of channels
    pub fn get_num_channels(&self) -> usize {
        if self.pattern.len() != 0 {
            self.pattern[0][0].len()
        } else {
            0
        }
    }

    /// get number of rows
    pub fn get_num_rows(&self, pat_idx: usize) -> usize {
        if self.pattern.len() != 0 {
            self.pattern[pat_idx].len()
        } else {
            0
        }
    }
}
