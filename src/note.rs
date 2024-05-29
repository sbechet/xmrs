use core::fmt::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// 8 octaves with notes
#[derive(Default, Serialize, Deserialize, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Note {
    /// No note
    #[default]
    None = 0,
    /// Octave 0
    C0 = 1,
    Cs0 = 2,
    D0 = 3,
    Ds0 = 4,
    E0 = 5,
    F0 = 6,
    Fs0 = 7,
    G0 = 8,
    Gs0 = 9,
    A0 = 10,
    As0 = 11,
    B0 = 12,
    /// Octave 1
    C1 = 13,
    Cs1 = 14,
    D1 = 15,
    Ds1 = 16,
    E1 = 17,
    F1 = 18,
    Fs1 = 19,
    G1 = 20,
    Gs1 = 21,
    A1 = 22,
    As1 = 23,
    B1 = 24,
    /// Octave 2
    C2 = 25,
    Cs2 = 26,
    D2 = 27,
    Ds2 = 28,
    E2 = 29,
    F2 = 30,
    Fs2 = 31,
    G2 = 32,
    Gs2 = 33,
    A2 = 34,
    As2 = 35,
    B2 = 36,
    /// Octave 3
    C3 = 37,
    Cs3 = 38,
    D3 = 39,
    Ds3 = 40,
    E3 = 41,
    F3 = 42,
    Fs3 = 43,
    G3 = 44,
    Gs3 = 45,
    A3 = 46,
    As3 = 47,
    B3 = 48,
    /// Octave 4
    C4 = 49,
    Cs4 = 50,
    D4 = 51,
    Ds4 = 52,
    E4 = 53,
    F4 = 54,
    Fs4 = 55,
    G4 = 56,
    Gs4 = 57,
    A4 = 58,
    As4 = 59,
    B4 = 60,
    /// Octave 5
    C5 = 61,
    Cs5 = 62,
    D5 = 63,
    Ds5 = 64,
    E5 = 65,
    F5 = 66,
    Fs5 = 67,
    G5 = 68,
    Gs5 = 69,
    A5 = 70,
    As5 = 71,
    B5 = 72,
    /// Octave 6
    C6 = 73,
    Cs6 = 74,
    D6 = 75,
    Ds6 = 76,
    E6 = 77,
    F6 = 78,
    Fs6 = 79,
    G6 = 80,
    Gs6 = 81,
    A6 = 82,
    As6 = 83,
    B6 = 84,
    /// Octave 7
    C7 = 85,
    Cs7 = 86,
    D7 = 87,
    Ds7 = 88,
    E7 = 89,
    F7 = 90,
    Fs7 = 91,
    G7 = 92,
    Gs7 = 93,
    A7 = 94,
    As7 = 95,
    B7 = 96,
    /// Stop note
    KeyOff = 97,
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let text = match self {
            Note::None => "---",
            // Octave 0
            Note::C0 => "C-0",
            Note::Cs0 => "C#0",
            Note::D0 => "D-0",
            Note::Ds0 => "D#0",
            Note::E0 => "E-0",
            Note::F0 => "F-0",
            Note::Fs0 => "F#0",
            Note::G0 => "G-0",
            Note::Gs0 => "G#0",
            Note::A0 => "A-0",
            Note::As0 => "A#0",
            Note::B0 => "B-0",
            // Octave 1
            Note::C1 => "C-1",
            Note::Cs1 => "C#1",
            Note::D1 => "D-1",
            Note::Ds1 => "D#1",
            Note::E1 => "E-1",
            Note::F1 => "F-1",
            Note::Fs1 => "F#1",
            Note::G1 => "G-1",
            Note::Gs1 => "G#1",
            Note::A1 => "A-1",
            Note::As1 => "A#1",
            Note::B1 => "B-1",
            // Octave 2
            Note::C2 => "C-2",
            Note::Cs2 => "C#2",
            Note::D2 => "D-2",
            Note::Ds2 => "D#2",
            Note::E2 => "E-2",
            Note::F2 => "F-2",
            Note::Fs2 => "F#2",
            Note::G2 => "G-2",
            Note::Gs2 => "G#2",
            Note::A2 => "A-2",
            Note::As2 => "A#2",
            Note::B2 => "B-2",
            // Octave 3
            Note::C3 => "C-3",
            Note::Cs3 => "C#3",
            Note::D3 => "D-3",
            Note::Ds3 => "D#3",
            Note::E3 => "E-3",
            Note::F3 => "F-3",
            Note::Fs3 => "F#3",
            Note::G3 => "G-3",
            Note::Gs3 => "G#3",
            Note::A3 => "A-3",
            Note::As3 => "A#3",
            Note::B3 => "B-3",
            // Octave 4
            Note::C4 => "C-4",
            Note::Cs4 => "C#4",
            Note::D4 => "D-4",
            Note::Ds4 => "D#4",
            Note::E4 => "E-4",
            Note::F4 => "F-4",
            Note::Fs4 => "F#4",
            Note::G4 => "G-4",
            Note::Gs4 => "G#4",
            Note::A4 => "A-4",
            Note::As4 => "A#4",
            Note::B4 => "B-4",
            // Octave 5
            Note::C5 => "C-5",
            Note::Cs5 => "C#5",
            Note::D5 => "D-5",
            Note::Ds5 => "D#5",
            Note::E5 => "E-5",
            Note::F5 => "F-5",
            Note::Fs5 => "F#5",
            Note::G5 => "G-5",
            Note::Gs5 => "G#5",
            Note::A5 => "A-5",
            Note::As5 => "A#5",
            Note::B5 => "B-5",
            // Octave 6
            Note::C6 => "C-6",
            Note::Cs6 => "C#6",
            Note::D6 => "D-6",
            Note::Ds6 => "D#6",
            Note::E6 => "E-6",
            Note::F6 => "F-6",
            Note::Fs6 => "F#6",
            Note::G6 => "G-6",
            Note::Gs6 => "G#6",
            Note::A6 => "A-6",
            Note::As6 => "A#6",
            Note::B6 => "B-6",
            // Octave 7
            Note::C7 => "C-7",
            Note::Cs7 => "C#7",
            Note::D7 => "D-7",
            Note::Ds7 => "D#7",
            Note::E7 => "E-7",
            Note::F7 => "F-7",
            Note::Fs7 => "F#7",
            Note::G7 => "G-7",
            Note::Gs7 => "G#7",
            Note::A7 => "A-7",
            Note::As7 => "A#7",
            Note::B7 => "B-7",
            // Stop note
            Note::KeyOff => "===",
        };
        write!(f, "{}", text)
    }
}

impl Note {
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.value() == 0
    }

    #[inline(always)]
    pub fn is_keyoff(&self) -> bool {
        self.value() == 97
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        let n: u8 = *self as u8;
        n > 0 && n < 97
    }

    #[inline(always)]
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
