use serde::{Deserialize, Serialize};

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
}

impl Instrument {
    // FinalVol = (FadeOutVol/65536)*(EnvelopeVol/64)*(GlobalVol/64)*(Vol/64)*Scale;
    // FinalPan = Pan + ( (EnvelopePan-32)*(128-Abs(Pan-128)) / 32 );
}
