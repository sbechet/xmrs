use crate::prelude::*;
use alloc::{vec, vec::Vec};

#[derive(Debug)]
pub struct PatternHelper {
    pub version: usize,
    pub songs: Vec<Vec<usize>>,
    pub channels: Vec<Vec<u8>>,
    pub tracks: Vec<Vec<u8>>,
}

impl PatternHelper {
    pub fn new(
        version: usize,
        songs: Vec<Vec<usize>>,
        channels: Vec<Vec<u8>>,
        tracks: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            version,
            songs,
            channels,
            tracks,
        }
    }

    fn get_track(&self, source: &Vec<u8>) -> Vec<PatternSlot> {
        let mut track: Vec<PatternSlot> = vec![];
        let mut index: usize = 0;
        let mut last_instr = 0;

        while source[index] != 255 {
            let mut current = PatternSlot::default();

            let length = source[index] & 0b0001_1111; // 0-31
            let release = (source[index] & 0b0010_0000) == 0;
            let append = (source[index] & 0b0100_0000) == 0;
            let instr_or_portamento = (source[index] & 0b1000_0000) != 0;

            if append {
                index += 1;
                if instr_or_portamento {
                    match self.version {
                        10 => {
                            if source[index] & 0b1000_0000 == 0 {
                                current.instrument = 1 + source[index] & 0b0111_1111;
                                last_instr = current.instrument;
                            } else {
                                // FIXME: can be E1 or E2?
                                let p = source[index] & 0b0111_1110;
                                if p != 0 {
                                    current.effect_parameter = p >> 1; // use max
                                    if source[index] & 1 == 0 {
                                        current.effect_type = 1; // portamento up
                                    } else {
                                        current.effect_type = 2; // portamento down
                                    }
                                }
                            }
                        }
                        15 => {
                            if source[index] & 0b1000_0000 == 0 {
                                current.instrument = 1 + source[index] & 0b0111_1111;
                                last_instr = current.instrument;
                            }
                        }
                        _ => {
                            if source[index] & 0b1000_0000 == 0 {
                                current.instrument = 1 + source[index] & 0b0111_1111;
                                last_instr = current.instrument;
                            } else {
                                let p: u16 = ((source[index] as u16 & 0b0011_1111) << 8)
                                    | source[index + 1] as u16;
                                index += 1;
                                if p != 0 {
                                    // FIXME: can i do better with that 6+8=14 bits type?
                                    current.effect_parameter = p as u8;
                                    if source[index] & 0b0100_0000 == 0 {
                                        current.effect_type = 1; // portamento up
                                    } else {
                                        current.effect_type = 2; // portamento down
                                    }
                                }
                            }
                        }
                    }
                    index += 1;
                }

                // correction of a table overflow in the original code
                let n = source[index] & 0b0111_1111;
                let note = if n > 8 * 12 {
                    // max is 96
                    match n {
                        96 => 0,   // self.regoffsets[0], for crazycomet
                        97 => 0,   // self.regoffsets[0], for commando
                        98 => 12,  // 8 for crazycomet, 16 for commando
                        100 => 3,  // 3 for commando, // self.patoffset[0],
                        104 => 65, // self.voicectrl[1],    // Good for Monty on the Run, Commando
                        105 => 65, // self.voicectrl[2],
                        107 => 6,  // self.instrnr[0],
                        127 => 0,
                        _ => n & 0b0011_1111, // FIXME: force limit 0..63
                    }
                } else {
                    n
                };
                current.note = (1 + note).try_into().unwrap();
            }

            // FIXME: last high bit from last byte is a bool about reset effect
            // let reset_effect = if self.version == 30 && source[index] & 0b1000_0000 == 0 {
            //     false
            // } else {
            //     true
            // };

            if release {
                if current.note == Note::None {
                    // current.note = Note::KeyOff;
                // FIXME: Hack, KeyOff currently forget instrument value?
                    current.volume = 0x10;
                } else {
                    current.volume = 0x50;
                }
                current.instrument = last_instr;
            }

            index += 1;
            track.push(current);

            if length != 0 {
                let current = PatternSlot::default();
                for _ in 0..length {
                    track.push(current);
                }
            }
        }
        return track;
    }

    fn get_tracks(&self) -> Vec<Vec<PatternSlot>> {
        let mut tracks: Vec<Vec<PatternSlot>> = vec![];
        for t in &self.tracks {
            tracks.push(self.get_track(t));
        }
        return tracks;
    }

    fn get_pattern_order(&self, song_number: usize) -> Vec<&Vec<u8>> {
        let mut pattern_order: Vec<&Vec<u8>> = vec![];
        for s_index in &self.songs[song_number] {
            pattern_order.push(&self.channels[*s_index]);
        }
        pattern_order
    }

    pub fn get_patterns(&self, song_number: usize) -> Vec<Pattern> {
        let tracks = self.get_tracks();
        let pattern_order = self.get_pattern_order(song_number);
        let po_len = pattern_order.len();
        let mut i_n: [usize; 3] = [0; 3];
        let mut patterns: Vec<Pattern> = vec![];

        loop {
            let mut trks: Vec<&Vec<PatternSlot>> = vec![];
            for k in 0..po_len {
                trks.push(&tracks[pattern_order[k][i_n[k]] as usize]);
            }
            // let mut trks_total_len = trks[0].len().max(trks[1].len().max(trks[2].len()));
            let mut trks_total_len = trks.iter().map(|sublist| sublist.len()).max().unwrap_or(0);
            let mut pattern: Vec<Vec<PatternSlot>> = vec![];
            let mut j: [usize; 3] = [0; 3];
            while trks_total_len != 0 {
                let mut line: Vec<PatternSlot> = vec![];
                for k in 0..po_len {
                    if j[k] >= trks[k].len() {
                        i_n[k] += 1;
                        if i_n[k] >= pattern_order[k].len() {
                            i_n[k] = 0;
                        } else {
                            if pattern_order[k][i_n[k]] == 254 {
                                //FIXME
                                pattern.push(line);
                                patterns.push(pattern);
                                return patterns;
                            }
                        }
                        j[k] = 0;
                        trks[k] = &tracks[pattern_order[k][i_n[k]] as usize];
                        if trks[k].len() > trks_total_len {
                            // trks_total_len += trks[k].len();
                            trks_total_len = trks[k].len();
                        }
                    }
                    line.push(trks[k][j[k]]);
                    j[k] += 1;
                }
                trks_total_len -= 1;
                pattern.push(line);
            }

            patterns.push(pattern);
            for k in 0..po_len {
                i_n[k] += 1;
                if i_n[k] >= pattern_order[k].len() {
                    i_n[k] = 0;
                } else {
                    if pattern_order[k][i_n[k]] == 254 {
                        return patterns;
                    }
                }
            }

            // last option to exit...
            if i_n[0] == 0 && i_n[1] == 0 && i_n[1] == 0 {
                return patterns;
            }
        }
    }

    pub fn cleanup_patterns(source: &Vec<Pattern>) -> (Vec<Pattern>, Vec<usize>) {
        let mut dest: Vec<Pattern> = Vec::new();
        let mut order: Vec<usize> = Vec::new();
        let mut seen_map: Vec<(Pattern, usize)> = Vec::new(); // Vec of (Pattern, index in dest)

        for pattern in source.iter() {
            if let Some(&(_, idx)) = seen_map.iter().find(|(p, _)| p == pattern) {
                order.push(idx);
            } else {
                let new_idx = dest.len();
                dest.push(pattern.clone());
                seen_map.push((pattern.clone(), new_idx));
                order.push(new_idx);
            }
        }

        (dest, order)
    }

    pub fn split_large_patterns(patterns: &mut Vec<Pattern>) {
        let max_size = 256;
        let mut i = 0;
    
        while i < patterns.len() {
            let current_pattern = &patterns[i];
            
            if current_pattern.len() > max_size {
                let num_splits = (current_pattern.len() + max_size - 1) / max_size;
                let mut new_patterns = Vec::new();
                
                for j in 0..num_splits {
                    let start = j * max_size;
                    let end = current_pattern.len().min(start + max_size);
                    let new_pattern = current_pattern[start..end].to_vec();
                    new_patterns.push(new_pattern);
                }
    
                patterns.splice(i..=i, new_patterns);
            } else {
                i += 1;
            }
        }
    }
}
