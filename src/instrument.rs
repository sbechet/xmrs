use serde::{Serialize, Deserialize};

use crate::envelope::Envelope;
use crate::vibrato::Vibrato;
use crate::sample::Sample;

//===========================================================================

/// Instrument Type
#[derive(Serialize, Deserialize, Debug)]
pub enum InstrumentType {
    /// No Instrument
    Empty,
    /// Historical XM Instrument
    Default(InstrDefault),
    /// MOS6581 SID Voice 
    Sid(InstrSid),
    /// Euclidian Rythm Instrument
    Euclidian(InstrEkn),
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
/// Euclidian Rythm Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrEkn {
    /// Pulsation k
    pub events: u8, 
    /// Duration n
    pub steps: u8,  
    /// Rotation
    pub rotation: u8,
    /// Instrument number
    pub instr: u8, 
}

impl Default for InstrEkn {
    fn default() -> Self {
        Self {
            events: 3,
            steps: 8,
            rotation: 0,
            instr: 0,
        }
    }
}

//===========================================================================
/// MOS6581 SID Voice
/// 
/*
 * synchronize & ring modulation:
 * - voice0: from voice2
 * - voice1: from voice0
 * - voice2: from voice1
 */
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SidVoice {
    //pub freq: u16,
    /// pulse wave duty cycle
    pub pw: u16,
    pub ctrl_noise: bool, // 7
    pub ctrl_pulse: bool, // 6
    pub ctrl_sawtooth: bool, // 5
    pub ctrl_triangle: bool, // 4
    pub ctrl_test: bool, // 3
    /// ring modulation (2->0, 0->1, 1->2)
    pub ctrl_rm: bool, // 2
    /// synchronize with (2->0, 0->1, 1->2)
    pub ctrl_sync: bool, // 1
    pub ctrl_gate: bool, // 0
    /// attack (0..15), decay (0..15)
    pub ad: u8,
    /// sustain (0..15), release (0..15)
    pub sr: u8,
}


impl Default for SidVoice {
    fn default() -> Self {
        Self {
            //freq: 0,
            pw: 0,
            ctrl_noise: false,
            ctrl_pulse: false,
            ctrl_sawtooth: false,
            ctrl_triangle: false,
            ctrl_test: false,
            ctrl_rm: false,
            ctrl_sync: false,
            ctrl_gate: false,
            ad: 0,
            sr: 0,
        }
    }
}

/// MOS6581 SID Instrument
#[derive(Serialize, Deserialize, Debug)]
pub struct InstrSid {
    pub voice: [SidVoice; 3],
    pub fc: u16,    // Filter cutoff frequency (0..2047) - Used by (Low, Band, High)Pass
    pub filter_resonance: u8, // u4 (0..15) - Amount of feedback a filter receives from itself
    pub filter_gate: [bool; 4], // 0 for voice0, ..., 3 for external input
    pub low_pass: bool,
    pub band_pass: bool,
    pub high_pass: bool,
    pub mute_voice3: bool,
    pub main_volume: u8,  // u4 (0..15)
}

impl Default for InstrSid {
    fn default() -> Self {
        Self {
            voice: [SidVoice::default(); 3],
            fc: 0,
            filter_resonance: 0,
            filter_gate: [false; 4],
            low_pass: false,
            band_pass: false,
            high_pass: false,
            mute_voice3: false,
            main_volume: 15,
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
