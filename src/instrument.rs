use serde::{Deserialize, Serialize};

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::instr_default::InstrDefault;
use crate::instr_ekn::InstrEkn;
use crate::instr_midi::InstrMidi;
use crate::instr_robsid::InstrRobSid;
use crate::instr_sid::InstrSid;

//===========================================================================

/// Instrument Type
#[derive(Default, Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    /// No Instrument
    #[default]
    Empty,
    /// Historical XM Instrument
    Default(Arc<InstrDefault>),
    /// Euclidian Rythm Instrument
    Euclidian(Arc<InstrEkn>),
    /// Midi Instrument
    Midi(Arc<InstrMidi>),
    /// MOS6581 SID Voice
    Sid(Arc<InstrSid>),
    /// Rob Hubbard Instrument,
    RobSid(Arc<InstrRobSid>),
}

/// Instrument with Steroid
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub name: String,
    pub instr_type: InstrumentType,
    pub muted: bool,
}
