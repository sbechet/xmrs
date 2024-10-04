use serde::{Deserialize, Serialize};

/// MOS6581 SID Voice
///
/// Synchronize & ring modulation:
/// - voice0: from voice2
/// - voice1: from voice0
/// - voice2: from voice1
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct SidVoice {
    //pub freq: u16,
    /// pulse wave duty cycle
    pub pw: u16,
    pub ctrl_noise: bool,    // 7
    pub ctrl_pulse: bool,    // 6
    pub ctrl_sawtooth: bool, // 5
    pub ctrl_triangle: bool, // 4
    pub ctrl_test: bool,     // 3
    /// ring modulation (2->0, 0->1, 1->2)
    pub ctrl_rm: bool, // 2
    /// synchronize with (2->0, 0->1, 1->2)
    pub ctrl_sync: bool, // 1
    pub ctrl_gate: bool,     // 0
    /// attack (0..15), decay (0..15)
    pub ad: u8,
    /// sustain (0..15), release (0..15)
    pub sr: u8,
}

impl SidVoice {
    pub fn update_from_ctrl_register(&mut self, ctrl: u8) {
        // 7 noise
        if ctrl & 0b1000_0000 != 0 {
            self.ctrl_noise = true;
        }
        // 6 pulse
        if ctrl & 0b0100_0000 != 0 {
            self.ctrl_pulse = true;
        }
        // 5 sawtooth
        if ctrl & 0b0010_0000 != 0 {
            self.ctrl_sawtooth = true;
        }
        // 4 triangle
        if ctrl & 0b0001_0000 != 0 {
            self.ctrl_triangle = true;
        }
        // 3 test
        if ctrl & 0b0000_1000 != 0 {
            self.ctrl_test = true;
        }
        // 2 ring modulation with voice X
        if ctrl & 0b0000_0100 != 0 {
            self.ctrl_rm = true;
        }
        // 1 synchronize with voice X
        if ctrl & 0b0000_0010 != 0 {
            self.ctrl_sync = true;
        }
        // 0 gate
        if ctrl & 0b0000_0001 != 0 {
            self.ctrl_gate = true;
        }
    }
}

/// MOS6581 SID Instrument
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrSid {
    pub voice: [SidVoice; 3],
    pub fc: u16, // Filter cutoff frequency (0..2047) - Used by (Low, Band, High)Pass
    pub filter_resonance: u8, // u4 (0..15) - Amount of feedback a filter receives from itself
    pub filter_gate: [bool; 4], // 0 for voice0, ..., 3 for external input
    pub low_pass: bool,
    pub band_pass: bool,
    pub high_pass: bool,
    pub mute_voice3: bool,
    pub main_volume: u8, // u4 (0..15)
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
