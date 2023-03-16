use serde::{Serialize, Deserialize};

use crate::envelope::Envelope;
use crate::vibrato::Vibrato;
use crate::sample::Sample;

use crate::instr_ekn::InstrEkn;
use crate::instr_midi::InstrMidi;
use crate::instr_sid::InstrSid;
use crate::instr_robsid::InstrRobSid;

//===========================================================================

/// Instrument Type
#[derive(Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    /// No Instrument
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

/// Generic Instrument with name
#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub name: String,
    pub instr_type: InstrumentType,
}

//===========================================================================
/// Historical XM Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrDefault {
    pub sample_for_note: Vec<u8>,  // =96
    pub volume_envelope: Envelope, // Envelope.points[].value: 0x00..0x3F
    pub panning_envelope: Envelope, // Envelope.points[].value: 0x00..0x3F
    pub vibrato: Vibrato,
    pub volume_fadeout: u16,    // 0x0000..0x0FFF
    pub sample: Vec<Sample>,
    pub midi: InstrMidi,
    pub midi_mute_computer: bool,
}

impl Default for InstrDefault {
    fn default() -> Self {
        Self {
            sample_for_note: vec![0; 96],
            volume_envelope: Envelope::default(),
            panning_envelope: Envelope::default(),
            vibrato: Vibrato::default(),
            volume_fadeout: 0,
            sample: vec![],
            midi: InstrMidi::default(),
            midi_mute_computer: false,
        }
    }
}

//===========================================================================

impl Default for Instrument {
    fn default() -> Self {
        Self {
            name: String::new(),
            instr_type: InstrumentType::Default(InstrDefault::default()),
        }
    }
}

impl Instrument {
    // FinalVol = (FadeOutVol/65536)*(EnvelopeVol/64)*(GlobalVol/64)*(Vol/64)*Scale;
    // FinalPan = Pan + ( (EnvelopePan-32)*(128-Abs(Pan-128)) / 32 );
}
