use crate::prelude::PatternSlot;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

pub struct XpPattern;

impl XpPattern {
    /// XP file must have 32 tracks per row
    pub fn save(pattern: &Vec<Vec<PatternSlot>>) -> Option<Vec<u8>> {
        let mut data: Vec<u8> = vec![];
        let version: u16 = 1;
        let nrow: u16 = pattern.len() as u16;

        if pattern[0].len() != 32 {
            return None;
        }

        data.append(&mut bincode::serde::encode_to_vec(&version, bincode::config::legacy()).unwrap());
        data.append(&mut bincode::serde::encode_to_vec(&nrow, bincode::config::legacy()).unwrap());

        for row in pattern {
            for ps in row {
                let mut d = ps.save_unpack();
                data.append(&mut d);
            }
        }

        Some(data)
    }
}
