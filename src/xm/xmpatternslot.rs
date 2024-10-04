use crate::note::Note;
use crate::patternslot::PatternSlot;
/// Original XM Pattern Slot
use bincode::error::DecodeError;

use alloc::vec::Vec;

pub type XmPatternSlot = PatternSlot;

impl XmPatternSlot {
    pub fn load(src: &[u8]) -> Result<(&[u8], XmPatternSlot), DecodeError> {
        let mut dst: [u8; 5] = [0; 5];
        let mut i = 0;
        let mut j = 0;

        let note = src[i];
        i += 1;
        if note & 0b1000_0000 != 0 {
            dst[j] = if note & 0b0000_0001 != 0 {
                i += 1;
                src[i - 1]
            } else {
                0
            };
            j += 1;
            dst[j] = if note & 0b0000_0010 != 0 {
                i += 1;
                src[i - 1]
            } else {
                0
            };
            j += 1;
            dst[j] = if note & 0b0000_0100 != 0 {
                i += 1;
                src[i - 1]
            } else {
                0
            };
            j += 1;
            dst[j] = if note & 0b0000_1000 != 0 {
                i += 1;
                src[i - 1]
            } else {
                0
            };
            j += 1;
            dst[j] = if note & 0b0001_0000 != 0 {
                i += 1;
                src[i - 1]
            } else {
                0
            };
        } else {
            dst[j] = note;
            j += 1;
            dst[j] = src[i];
            i += 1;
            j += 1;
            dst[j] = src[i];
            i += 1;
            j += 1;
            dst[j] = src[i];
            i += 1;
            j += 1;
            dst[j] = src[i];
            i += 1;
        }

        Ok((
            &src[i..],
            XmPatternSlot {
                note: {
                    if dst[0] == 97 {
                        // Special case: we don't want to use 97, because we want more octaves...
                        Note::KeyOff
                    } else {
                        match Note::try_from(dst[0]) {
                            Ok(n) => n,
                            Err(_e) => Note::None,
                        }
                    }
                },
                instrument: dst[1],
                volume: dst[2],
                effect_type: dst[3],
                effect_parameter: dst[4],
            },
        ))
    }

    pub fn save_unpack(&self) -> Vec<u8> {
        let mut bytes: [u8; 5] = [0; 5];
        bytes[0] = {
            if self.note.is_keyoff() {
                97
            } else {
                self.note.into()
            }
        };
        bytes[1] = self.instrument;
        bytes[2] = self.volume;
        bytes[3] = self.effect_type;
        bytes[4] = self.effect_parameter;
        return bytes.to_vec();
    }

    pub fn save(&self) -> Vec<u8> {
        let mut bytes: [u8; 5] = [0; 5];
        bytes[0] = {
            if self.note.is_keyoff() {
                97
            } else {
                self.note.into()
            }
        };
        bytes[1] = self.instrument;
        bytes[2] = self.volume;
        bytes[3] = self.effect_type;
        bytes[4] = self.effect_parameter;

        let mut dst: [u8; 5] = [0; 5];
        let mut pack_bits = 0;
        let mut i = 1;
        if bytes[0] > 0 {
            pack_bits |= 0b0001;
            dst[i] = bytes[0];
            i += 1;
        } // note
        if bytes[1] > 0 {
            pack_bits |= 0b0010;
            dst[i] = bytes[1];
            i += 1;
        } // instrument
        if bytes[2] > 0 {
            pack_bits |= 0b0100;
            dst[i] = bytes[2];
            i += 1;
        } // volume
        if bytes[3] > 0 {
            pack_bits |= 0b1000;
            dst[i] = bytes[3];
            i += 1;
        } // effect type

        if pack_bits == 15 {
            // first four bits set? no packing needed.
            return bytes.to_vec();
        }

        if bytes[4] > 0 {
            pack_bits |= 16;
            dst[i] = bytes[4];
            i += 1;
        } // effect parameter
        dst[0] = pack_bits | 0b1000_0000;
        dst[0..i].to_vec()
    }
}
