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

impl Sample {
    /// return sample length
    pub fn len(&self) -> usize {
        match &self.data {
            SampleDataType::Depth8(v) => v.len(),
            SampleDataType::Depth16(v) => v.len(),
        }
    }

    /// return sample at seek
    pub fn at(&self, seek: usize) -> f32 {
        match &self.data {
            SampleDataType::Depth8(v) => v[seek] as f32 / 128.0,
            SampleDataType::Depth16(v) => v[seek] as f32 / 32768.0,
        }
    }

    /// return sample size (8 or 16 bits)
    pub fn bits(&self) -> u8 {
        match &self.data {
            SampleDataType::Depth8(_) => 8,
            SampleDataType::Depth16(_) => 16,
        }
    }
}
