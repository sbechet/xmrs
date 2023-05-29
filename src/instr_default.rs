use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::sync::Arc;

use crate::envelope::Envelope;
use crate::instr_midi::InstrMidi;
use crate::sample::Sample;
use crate::vibrato::Vibrato;

/// Historical XM Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrDefault {
    #[serde(with = "BigArray")]
    pub sample_for_note: [u8; 96],
    pub volume_envelope: Arc<Envelope>, // Envelope.points[].value: 0x00..0x3F
    pub panning_envelope: Arc<Envelope>, // Envelope.points[].value: 0x00..0x3F
    pub vibrato: Arc<Vibrato>,
    pub volume_fadeout: f32, // 0.0..1.0
    pub sample: Vec<Arc<Sample>>,
    pub midi: InstrMidi,
    pub midi_mute_computer: bool,
}

impl Default for InstrDefault {
    fn default() -> Self {
        Self {
            sample_for_note: [0; 96],
            volume_envelope: Arc::new(Envelope::default()),
            panning_envelope: Arc::new(Envelope::default()),
            vibrato: Arc::new(Vibrato::default()),
            volume_fadeout: 0.0,
            sample: vec![],
            midi: InstrMidi::default(),
            midi_mute_computer: false,
        }
    }
}
