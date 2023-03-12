use serde::{Serialize, Deserialize};

use crate::envelope::Envelope;
use crate::vibrato::Vibrato;
use crate::sample::Sample;


//===========================================================================

#[derive(Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    Empty,
    Default(InstrDefault),
    Sid(InstrSid),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument {
    pub name: String,
    pub instr_type: InstrumentType,
}

//===========================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrDefault {
    pub sample_for_note: Vec<u8>,  // =96
    pub volume_envelope: Envelope, // Envelope.points[].value: 0x00..0x3F
    pub panning_envelope: Envelope, // Envelope.points[].value: 0x00..0x3F
    pub vibrato: Vibrato,
    pub volume_fadeout: u16,    // 0x0000..0x0FFF
    pub sample: Vec<Sample>,
}


impl Default for InstrDefault {
    fn default() -> Self {
        Self {
            sample_for_note: vec![0; 96],
            volume_envelope: Envelope::default(),
            panning_envelope: Envelope::default(),
            vibrato: Vibrato::default(),
            volume_fadeout: 0,
            sample: vec![]
        }
    }
}

//===========================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrSid {

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