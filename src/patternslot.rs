use crate::note::Note;
use serde::{Deserialize, Serialize};
use std::fmt::*;

/// A typical pattern slot
#[derive(Serialize, Deserialize, Copy, Clone)]
#[repr(C)]
pub struct PatternSlot {
    pub note: Note,
    /// 0: none, 1-128
    pub instrument: u8,
    /// 0..64, 255
    pub volume: u8,
    pub effect_type: u8,
    pub effect_parameter: u8,
}

impl Default for PatternSlot {
    fn default() -> Self {
        PatternSlot {
            note: Note::None,
            instrument: 0,
            volume: 0,
            effect_type: 0,
            effect_parameter: 0,
        }
    }
}

impl Debug for PatternSlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let v = if self.volume == 0 {
            '-'
        } else {
            std::char::from_digit(u32::from(self.volume & 0x0f), 16).unwrap()
        };
        write!(
            f,
            "[{:?} {:>2X} {:>1}{} {}{:02X}]",
            self.note,
            self.instrument,
            self.volume_letter(),
            v,
            self.effect_letter(),
            self.effect_parameter
        )
    }
}

impl PatternSlot {
    pub fn has_tone_portamento(&self) -> bool {
        self.effect_type == 3 || self.effect_type == 5 || self.volume >> 4 == 0x0F
    }

    pub fn has_arpeggio(&self) -> bool {
        self.effect_type == 0 && self.effect_parameter != 0
    }

    pub fn has_vibrato(&self) -> bool {
        self.effect_type == 4 || self.effect_type == 6 || self.volume >> 4 == 0x0B
    }

    pub fn has_retrigger_note_empty(&self) -> bool {
        self.effect_type == 0xE && self.effect_parameter == 0x90
    }

    pub fn volume_letter(&self) -> char {
        match self.volume >> 4 {
            0x0 => '-',
            0x1 => '1',
            0x2 => '2',
            0x3 => '3',
            0x4 => '4',
            0x5 => '5',
            0x6 => '-',
            0x7 => '+',
            0x8 => 'D',
            0x9 => 'U',
            0xA => 'S',
            0xB => 'V',
            0xC => 'P',
            0xD => 'L',
            0xE => 'R',
            0xF => 'M',
            _ => ' ',
        }
    }

    pub fn effect_letter(&self) -> char {
        match self.effect_type {
            0x0 => '0',
            0x1 => '1',
            0x2 => '2',
            0x3 => '3',
            0x4 => '4',
            0x5 => '5',
            0x6 => '6',
            0x7 => '7',
            0x8 => ' ',
            0x9 => ' ',
            0xA => 'A',
            0xB => 'B',
            0xC => ' ',
            0xD => 'D',
            0xE => 'E',
            0xF => 'F',
            0x10 => 'G',
            0x11 => 'H',
            0x12 => ' ',
            0x13 => ' ',
            0x14 => 'K',
            0x15 => 'L',
            0x16 => ' ',
            0x17 => ' ',
            0x18 => ' ',
            0x19 => 'P',
            0x1A => ' ',
            0x1B => 'R',
            0x1C => ' ',
            0x1D => 'T',
            0x1E => ' ',
            0x1F => ' ',
            0x20 => ' ',
            0x21 => 'X',
            _ => ' ',
        }
    }
}
