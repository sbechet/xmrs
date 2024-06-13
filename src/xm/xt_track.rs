use crate::prelude::PatternSlot;

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

pub struct XtTrack;

impl XtTrack {
    /// Here we use `Vec<PatternSlot>` like a track _not_ like a Pattern row!
    pub fn save(track: &Vec<PatternSlot>) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let version: u16 = 1;
        let nrow: u16 = track.len() as u16;
        data.append(&mut bincode::serialize(&version).unwrap());
        data.append(&mut bincode::serialize(&nrow).unwrap());
        for xmps in track {
            let mut d = xmps.save_unpack();
            data.append(&mut d);
        }
        data
    }
}
