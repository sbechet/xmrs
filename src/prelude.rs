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
    instr_opl::{InstrOpl, MdiInstr, MdiOpl},
    instr_robsid::InstrRobSid,
    instr_sid::InstrSid,
    instr_vibrato::{InstrVibrato, Waveform},
    instrument::{Instrument, InstrumentType},
    module::{Module, Pattern, Row, DEFAULT_PATTERN_LENGTH, MAX_NUM_ROWS},
    note::Note,
    patternslot::PatternSlot,
    period_helper::{FrequencyType, PeriodHelper},
    sample::{LoopType, Sample, SampleDataType},
};
