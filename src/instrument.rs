use serde::{Deserialize, Serialize};

use crate::instr_default::InstrDefault;
use crate::instr_ekn::InstrEkn;
use crate::instr_midi::InstrMidi;
use crate::instr_robsid::InstrRobSid;
use crate::instr_sid::InstrSid;

#[cfg(not(feature = "std"))]
use alloc::string::String;

//===========================================================================

/// Instrument Type
#[derive(Default, Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    /// No Instrument
    #[default]
    Empty,
    /// Historical XM Instrument
    Default(InstrDefault),
    /// Euclidian Rythm Instrument
    Euclidian(InstrEkn),
    /// Midi Instrument
    Midi(InstrMidi),
    /// MOS6581 SID Voice
    Sid(InstrSid),
    /// Rob Hubbard Instrument,
    RobSid(InstrRobSid),
}

/// Instrument with Steroid
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub name: String,
    pub instr_type: InstrumentType,
    pub muted: bool,
}
