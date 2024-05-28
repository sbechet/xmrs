use bincode::error::{DecodeError, EncodeError};
use serde::{Deserialize, Serialize};

use libflate::deflate::*;

// With std, this is equal to std::io::{Read, Write}
use core2::io::{Read, Write};

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::instrument::Instrument;
use crate::patternslot::PatternSlot;

pub const DEFAULT_PATTERN_LENGTH: usize = 64;
pub const MAX_NUM_ROWS: usize = 256;

/// Historical Frequencies to load old data. Default is Linear.
#[derive(bincode::Encode, Serialize, bincode::Decode, Deserialize, Clone, Copy, Debug)]
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
#[derive(bincode::Encode, Serialize, bincode::Decode, Deserialize, Debug)]
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
    pub fn load(data: &[u8]) -> Result<Module, Box<DecodeError>> {
        let version = env!("CARGO_PKG_VERSION_MAJOR");
        let mut header: [u8; 5] = *b"XMrs ";
        header[4] = version.as_bytes()[0];

        let ver_data = &data[0..5];
        let real_data = &data[5..];
        if ver_data != header {
            Err(Box::new(DecodeError::Other(
                "Bad Module version",
            )))
        } else {
            let mut decoder = Decoder::new(real_data);
            let mut decoded_data = Vec::new();
            decoder.read_to_end(&mut decoded_data).unwrap();

            Ok(bincode::serde::decode_from_slice(&decoded_data, bincode::config::legacy())?.0)
        }
    }

    /// Save module using bincode
    pub fn save(&self) -> Result<Vec<u8>, Box<EncodeError>> {
        let version = env!("CARGO_PKG_VERSION_MAJOR");
        let mut header: [u8; 5] = *b"XMrs ";
        header[4] = version.as_bytes()[0];

        let mut ser_all = header.to_vec();

        // EncodeError doesn't support core2, only contains Io variant when std present
        #[cfg(feature = "std")]
        let io_error_wrap = |e| Box::new(EncodeError::Io{inner:e, index:0});
        #[cfg(not(feature = "std"))]
        let io_error_wrap = |_| Box::new(EncodeError::Other("LZ77 compreession failed"));
        let ser_mod1 = bincode::serde::encode_to_vec(&self, bincode::config::legacy())?;
        let mut encoder = Encoder::new(Vec::new());
        encoder.write_all(&ser_mod1).map_err(io_error_wrap)?;
        let mut ser_mod2 = encoder.finish().into_result().map_err(io_error_wrap)?;

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
