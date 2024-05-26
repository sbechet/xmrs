use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[cfg(feature = "compress")]
use bincode::ErrorKind;
#[cfg(feature = "compress")]
use libflate::deflate::*;
#[cfg(feature = "compress")]
use std::io::{Read, Write};


use crate::instrument::Instrument;
use crate::patternslot::PatternSlot;

pub const DEFAULT_PATTERN_LENGTH: usize = 64;
pub const MAX_NUM_ROWS: usize = 256;

/// Historical Frequencies to load old data. Default is Linear.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum FrequencyType {
    AmigaFrequencies,
    LinearFrequencies,
}

impl Default for FrequencyType {
    fn default() -> Self {
        Self::LinearFrequencies
    }
}

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
    pub restart_position: u16,
    pub default_tempo: u16,
    pub default_bpm: u16,
    /// Defines the exact order for the patterns playback
    pub pattern_order: Vec<u8>,
    pub pattern: Vec<Arc<Pattern>>,
    /// Instrument 1 has index 0, instrument 2 has index 1, etc.
    pub instrument: Vec<Arc<Instrument>>,
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
    /// Load module using bincode
    #[cfg(feature = "compress")]
    pub fn load(data: &[u8]) -> Result<Module, Box<ErrorKind>> {
        let version = env!("CARGO_PKG_VERSION_MAJOR");
        let mut header: [u8; 5] = *b"XMrs ";
        header[4] = version.as_bytes()[0];

        let ver_data = &data[0..5];
        let real_data = &data[5..];
        if ver_data != header {
            Err(Box::new(ErrorKind::Custom(
                "Bad Module version".to_string(),
            )))
        } else {
            let mut decoder = Decoder::new(real_data);
            let mut decoded_data = Vec::new();
            decoder.read_to_end(&mut decoded_data).unwrap();

            Ok(bincode::deserialize(&decoded_data)?)
        }
    }

    /// Save module using bincode
    #[cfg(feature = "compress")]
    pub fn save(&self) -> Result<Vec<u8>, Box<ErrorKind>> {
        let version = env!("CARGO_PKG_VERSION_MAJOR");
        let mut header: [u8; 5] = *b"XMrs ";
        header[4] = version.as_bytes()[0];

        let mut ser_all = header.to_vec();

        let ser_mod1 = bincode::serialize(&self)?;
        let mut encoder = Encoder::new(Vec::new());
        encoder.write_all(&ser_mod1)?;
        let mut ser_mod2 = encoder.finish().into_result()?;

        ser_all.append(&mut ser_mod2);

        Ok(ser_all)
    }

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
