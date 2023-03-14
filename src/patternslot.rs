use serde::{Serialize, Deserialize};
use num_enum::{ TryFromPrimitive, IntoPrimitive };

#[derive(Serialize, Deserialize, TryFromPrimitive, IntoPrimitive, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Note {
    /// No note
    None = 0,
    /// Octave 0
    C0=1,     Cs0=2,     D0=3,     Ds0=4,     E0=5,     F0=6,     Fs0=7,     G0=8,     Gs0=9,     A0=10,    As0=11,    B0=12,
    /// Octave 1
    C1=13,    Cs1=14,    D1=15,    Ds1=16,    E1=17,    F1=18,    Fs1=19,    G1=20,    Gs1=21,    A1=22,    As1=23,    B1=24,
    /// Octave 2
    C2=25,    Cs2=26,    D2=27,    Ds2=28,    E2=29,    F2=30,    Fs2=31,    G2=32,    Gs2=33,    A2=34,    As2=35,    B2=36,
    /// Octave 3
    C3=37,    Cs3=38,    D3=39,    Ds3=40,    E3=41,    F3=42,    Fs3=43,    G3=44,    Gs3=45,    A3=46,    As3=47,    B3=48,
    /// Octave 4
    C4=49,    Cs4=50,    D4=51,    Ds4=52,    E4=53,    F4=54,    Fs4=55,    G4=56,    Gs4=57,    A4=58,    As4=59,    B4=60,
    /// Octave 5
    C5=61,    Cs5=62,    D5=63,    Ds5=64,    E5=65,    F5=66,    Fs5=67,    G5=68,    Gs5=69,    A5=70,    As5=71,    B5=72,
    /// Octave 6
    C6=73,    Cs6=74,    D6=75,    Ds6=76,    E6=77,    F6=78,    Fs6=79,    G6=80,    Gs6=81,    A6=82,    As6=83,    B6=84,
    /// Octave 7
    C7=85,    Cs7=86,    D7=87,    Ds7=88,    E7=89,    F7=90,    Fs7=91,    G7=92,    Gs7=93,    A7=94,    As7=95,    B7=96,
    /// Stop note
    KeyOff = 97,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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
