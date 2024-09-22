use serde::{Deserialize, Serialize};

/// MDI_OPLREGS structure
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MdiOpl {
    /// 2 bits, Key scaling level (OPL: 0x40, bits 6-7)
    pub ksl: u8,
    /// 4 bits, Frequency multiplier (OPL: 0x20, bits 0-3)
    pub multiple: u8,
    /// 3 bits [op 0 only, op 1 ignored] (OPL: 0xC0, bits 1-3)
    pub feedback: u8,
    /// 4 bits (OPL: 0x60, bits 4-7)
    pub attack: u8,
    /// 4 bits (OPL: 0x80, bits 4-7)
    pub sustain: u8,
    /// Envelope gain (OPL: 0x20, bit 5)
    pub eg: bool,
    /// 4 bits (OPL: 0x60, bits 0-3)
    pub decay: u8,
    /// 4 bits (OPL: 0x80, bits 0-3)
    pub release: u8,
    /// 6 bits [0=loudest, 63=silent] (OPL: 0x40, bits 0-5)
    pub total_level: u8,
    /// Amplitude modulation (Tremolo) (OPL: 0x20, bit 7)
    pub am: bool,
    /// Frequency vibrato (OPL: 0x20, bit 6)
    pub vib: bool,
    /// Key scaling/envelope rate (OPL: 0x20, bit 4)
    pub ksr: bool,
    /// Key scaling/envelope rate (OPL: 0x20, bit 4)
    pub con: bool,
}

#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MdiInstr {
    pub modulator: MdiOpl, // Register values for the Modulator operator (op 0)
    pub carrier: MdiOpl,   // Register values for the Carriere operator (op 1)
    pub modulator_wave_select: u8, // (OPL: 0xE0)
    pub carrier_wave_select: u8, // (OPL: 0xE0)
}

/// Yamaha OPL with module compatibility
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct InstrOpl {
    pub element: MdiInstr,
    pub volume: u8, // 0-63
    /// [-1..1]
    pub finetune: f32,
    /// [-96..95] with 0 <=> C-4
    pub relative_note: i8,
}
