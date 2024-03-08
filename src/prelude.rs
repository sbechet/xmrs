/// The Xmrs Prelude.
///
/// The purpose of this module is to alleviate imports of Xmrs module parts
///
/// ```
/// #![allow(unused_imports)]
/// use xmrs::prelude::*;
/// ```
///
pub use crate::{
    envelope::{Envelope, EnvelopePoint},
    instr_default::InstrDefault,
    instr_ekn::InstrEkn,
    instr_midi::InstrMidi,
    instr_robsid::InstrRobSid,
    instr_sid::InstrSid,
    instrument::{Instrument, InstrumentType},
    module::{FrequencyType, Module, Pattern, Row, DEFAULT_PATTERN_LENGTH, MAX_NUM_ROWS},
    note::Note,
    patternslot::PatternSlot,
    sample::{LoopType, Sample, SampleDataType},
    instr_vibrato::{InstrVibrato, Waveform},
};
