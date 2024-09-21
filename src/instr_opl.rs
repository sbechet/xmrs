use serde::{Deserialize, Serialize};

/// MDI_OPLREGS structure
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MdiOpl {
    /// 2 bits, Key scaling level (OPL: 0x40, bits 6-7)
    ksl: u8,
    /// 4 bits, Frequency multiplier (OPL: 0x20, bits 0-3)
    multiple: u8,
    /// 3 bits [op 0 only, op 1 ignored] (OPL: 0xC0, bits 1-3)
    feedback: u8,
    /// 4 bits (OPL: 0x60, bits 4-7)
    attack: u8,
    /// 4 bits (OPL: 0x80, bits 4-7)
    sustain: u8,
    /// Envelope gain (OPL: 0x20, bit 5)
    eg: bool,
    /// 4 bits (OPL: 0x60, bits 0-3)
    decay: u8,
    /// 4 bits (OPL: 0x80, bits 0-3)
    release: u8,
    /// 6 bits [0=loudest, 63=silent] (OPL: 0x40, bits 0-5)
    total_level: u8,
    /// Amplitude modulation (Tremolo) (OPL: 0x20, bit 7)
    am: bool,
    /// Frequency vibrato (OPL: 0x20, bit 6)
    vib: bool,
    /// Key scaling/envelope rate (OPL: 0x20, bit 4)
    ksr: bool,
    /// Key scaling/envelope rate (OPL: 0x20, bit 4)
    con: bool,
}

#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MdiInstr {
    modulator: MdiOpl,         // Register values for the Modulator operator (op 0)
    carrier: MdiOpl,           // Register values for the Carriere operator (op 1)
    modulator_wave_select: u8, // (OPL: 0xE0)
    carrier_wave_select: u8,   // (OPL: 0xE0)
}

/// Yamaha OPL with module compatibility
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug)]
pub struct InstrOpl {
    element: MdiInstr,
    volume: u8, // 0-63
    c2spd: u32, // sample rate for middle-C (note (C-4), default 8363.
}
