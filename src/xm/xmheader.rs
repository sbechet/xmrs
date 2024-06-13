/// Original XM Header
use bincode::ErrorKind;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use super::serde_helper::{deserialize_string_17, serialize_string_17};
use super::serde_helper::{deserialize_string_20, serialize_string_20};

use crate::module::{FrequencyType, Module};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum XmFlagType {
    XmAmigaFrequencies = 0,
    XmLinearFrequencies = 1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmHeader {
    #[serde(
        deserialize_with = "deserialize_string_17",
        serialize_with = "serialize_string_17"
    )]
    id_text: String, // "Extended module: "
    #[serde(
        deserialize_with = "deserialize_string_20",
        serialize_with = "serialize_string_20"
    )]
    pub name: String,
    right_arrow: u8, // 0x1A on IBM437 charset matches →. For `COMMAND.COM` `TYPE` command, it is Ctrl-Z or EOF. Yes: a CP/M heritage...so old!
    #[serde(
        deserialize_with = "deserialize_string_20",
        serialize_with = "serialize_string_20"
    )]
    pub tracker_name: String,
    pub version_number: u16,
    pub header_size: u32, // 20 (starting from here to PatternOrder start) + pattern size (default 256, can be less...or more)
    song_length: u16,     // pattern order table "size" in bytes
    pub restart_position: u16, // PatternOrder index
    pub number_of_channels: u16, // 0..32/64
    pub number_of_patterns: u16, // 1..256
    pub number_of_instruments: u16, // 0..128
    pub flags: XmFlagType,
    pub default_tempo: u16,
    pub default_bpm: u16,
}

impl Default for XmHeader {
    fn default() -> Self {
        Self {
            id_text: "Extended Module: ".to_string(),
            name: "".to_string(),
            right_arrow: 0x1A,
            tracker_name: "XMrs".to_string(), // or "Fasttracker II clone"
            version_number: 0x0104,           // minimal version number supported
            header_size: 20,
            song_length: 0,
            restart_position: 0,
            number_of_channels: 8,
            number_of_patterns: 0,
            number_of_instruments: 0,
            flags: XmFlagType::XmLinearFrequencies,
            default_tempo: 6,
            default_bpm: 125,
        }
    }
}

impl XmHeader {
    /* return like nom (&[u8], (XmHeader, PatternOrder) ) */
    pub fn load(ser_xmheader: &[u8]) -> Result<(&[u8], XmHeader, Vec<u8>), Box<ErrorKind>> {
        match bincode::deserialize::<XmHeader>(ser_xmheader) {
            Ok(xmh) => {
                if xmh.id_text != "Extended Module:" {
                    return Err(Box::new(ErrorKind::Custom(
                        "Not an Extended Module?".to_string(),
                    )));
                }
                match xmh.get_pattern_order(&ser_xmheader[80..]) {
                    Ok((data, pattern_order)) => Ok((data, xmh, pattern_order)),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn get_pattern_order<'a>(&self, data: &'a [u8]) -> Result<(&'a [u8], Vec<u8>), Box<ErrorKind>> {
        let pattern_order_and_maybe_more_len: usize = self.header_size as usize - 20;
        if data.len() >= pattern_order_and_maybe_more_len
            && self.song_length as usize <= pattern_order_and_maybe_more_len
        {
            let pattern_order: Vec<u8> = data[0..self.song_length as usize].to_vec();
            Ok((&data[pattern_order_and_maybe_more_len..], pattern_order))
        } else {
            Err(serde::de::Error::custom("XmHeader.header_size too big?"))
        }
    }

    /// Extract XmHeader and pattern_order from Module
    pub fn from_module(module: &Module) -> (Self, Vec<u8>) {
        let mut xmh = XmHeader {
            name: module.name.clone(),
            song_length: module.pattern_order.len() as u16,
            restart_position: module.restart_position,
            number_of_channels: if !module.pattern.is_empty() {
                module.pattern[0][0].len() as u16
            } else {
                8
            },
            number_of_patterns: module.pattern.len() as u16,
            number_of_instruments: module.instrument.len() as u16,
            flags: match module.frequency_type {
                FrequencyType::LinearFrequencies => XmFlagType::XmLinearFrequencies,
                FrequencyType::AmigaFrequencies => XmFlagType::XmAmigaFrequencies,
            },
            default_tempo: module.default_tempo,
            default_bpm: module.default_bpm,
            ..Default::default()
        };
        let pattern_order = module.pattern_order.clone();
        xmh.header_size += 256;

        (xmh, pattern_order)
    }
}
