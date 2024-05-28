use serde::{Deserialize, Serialize};

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
