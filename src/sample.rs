use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

/// How to play sample
#[derive(Default, Serialize, Deserialize, Copy, Clone, IntoPrimitive, TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum LoopType {
    #[default]
    No = 0,
    Forward = 1,
    PingPong = 2,
}

/// is sample recorded with 8 or 16 bits depth
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SampleDataType {
    Depth8(Vec<i8>),
    Depth16(Vec<i16>),
}

/// A Real Data sample
#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    pub name: String,
    pub loop_start: u32,
    pub loop_length: u32,
    pub volume: u8,
    pub finetune: i8,
    pub flags: LoopType,
    pub panning: u8,
    pub relative_note: i8,
    pub data: SampleDataType,
}
