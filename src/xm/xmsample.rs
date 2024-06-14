/// Original XM Sample
use bincode::ErrorKind;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::{vec, vec::Vec};

use super::helper::*;
use super::serde_helper::{deserialize_string_22, serialize_string_22};
use crate::instrument::{Instrument, InstrumentType};
use crate::sample::{LoopType, Sample, SampleDataType};

pub const XMSAMPLE_HEADER_SIZE: usize = 40;

#[derive(Default, Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct XmSampleHeader {
    length: u32,
    loop_start: u32,
    loop_length: u32,
    volume: u8,
    finetune: i8,
    flags: u8,
    panning: u8,
    relative_note: i8,
    reserved: u8,
    #[serde(
        deserialize_with = "deserialize_string_22",
        serialize_with = "serialize_string_22"
    )]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmSample {
    header: XmSampleHeader,
    data: Option<SampleDataType>,
}

impl Default for XmSample {
    fn default() -> Self {
        XmSample {
            header: XmSampleHeader::default(),
            data: None,
        }
    }
}

impl XmSample {
    pub fn load(data: &[u8]) -> Result<(&[u8], XmSample), Box<ErrorKind>> {
        let sh = bincode::deserialize::<XmSampleHeader>(data)?;
        // Now create XmSample
        let xms = XmSample {
            header: sh,
            data: None,
        };
        Ok((&data[XMSAMPLE_HEADER_SIZE..], xms))
    }

    pub fn add_sample<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], Box<ErrorKind>> {
        let data_len: usize = self.header.length as usize;
        let slice = &data[..data_len];

        let d3 = if self.header.flags & 0b0001_0000 != 0 {
            // 16 bits data
            let sample = u8_slice_to_vec_u16(slice);
            let sample2 = delta16_to_sample(sample);
            SampleDataType::Depth16(sample2)
        } else {
            // 8 bits data
            let sample = slice.to_vec();
            let sample2 = delta8_to_sample(sample);
            SampleDataType::Depth8(sample2)
        };
        self.data = Some(d3);

        Ok(&data[data_len..])
    }

    pub fn save(&mut self) -> Result<Vec<u8>, Box<ErrorKind>> {
        self.header.length = match &self.data {
            Some(SampleDataType::Depth8(d)) => d.len() as u32,
            Some(SampleDataType::Depth16(d)) => {
                self.header.flags |= 0b0001_0000;
                2 * d.len() as u32
            }
            None => 0,
        };
        let h = bincode::serialize(&self.header)?;
        Ok(h)
    }

    /// You must call save() before to save good length size to header
    pub fn save_sample(&mut self) -> Result<Vec<u8>, Box<ErrorKind>> {
        let d = match &self.data {
            Some(SampleDataType::Depth8(d)) => sample8_to_delta(d),
            Some(SampleDataType::Depth16(d)) => {
                let d = sample16_to_delta(d);
                vec_u16_to_u8_slice(d)
            }
            None => vec![],
        };
        Ok(d)
    }

    pub fn to_sample(&self) -> Sample {
        let mut loop_start = self.header.loop_start;
        let mut loop_length = self.header.loop_length;

        if let Some(SampleDataType::Depth16(_)) = &self.data {
            loop_start >>= 1;
            loop_length >>= 1;
        }

        /* Fix invalid loop definitions */
        let sample_length = self.len();
        if sample_length == 0 {
            loop_start = 0;
            loop_length = 0;
        } else {
            if loop_start >= sample_length {
                loop_start = sample_length - 1;
            }
            if loop_length > sample_length - loop_start {
                loop_length = sample_length - loop_start;
            }
        }

        let data: SampleDataType = match &self.data {
            Some(d) => d.clone(),
            None => SampleDataType::Depth8(vec![]),
        };

        Sample {
            name: self.header.name.clone(),
            loop_start: loop_start,
            loop_length: loop_length,
            volume: self.header.volume as f32 / 64.0,
            finetune: (self.header.finetune as f32 / 127.0).clamp(-1.0, 1.0),
            flags: match self.header.flags & 0b000000_11 {
                1 => LoopType::Forward,
                2 => LoopType::PingPong,
                3 => LoopType::PingPong,
                _ => LoopType::No,
            },
            panning: self.header.panning as f32 / 255.0,
            relative_note: self.header.relative_note,
            data: data,
        }
    }

    pub fn from_instr(i: &Instrument) -> Vec<XmSample> {
        let mut output: Vec<XmSample> = vec![];
        if let InstrumentType::Default(id) = &i.instr_type {
            for s in &id.sample {
                let mut loop_start = s.loop_start;
                let mut loop_length = s.loop_length;

                if let SampleDataType::Depth16(_) = &s.data {
                    loop_start <<= 1;
                    loop_length <<= 1;
                }

                let mut xms = XmSample::default();
                xms.header.length = match &s.data {
                    SampleDataType::Depth8(d) => d.len() as u32,
                    SampleDataType::Depth16(d) => 2 * d.len() as u32,
                };
                xms.header.loop_start = loop_start;
                xms.header.loop_length = loop_length;
                xms.header.volume = (s.volume * 64.0) as u8;
                xms.header.finetune = (s.finetune * 127.0) as i8;
                xms.header.flags = s.flags.into();
                xms.header.panning = (s.panning * 255.0) as u8;
                xms.header.relative_note = s.relative_note;
                xms.header.name = s.name.clone();
                xms.data = Some(s.data.clone());
                output.push(xms);
            }
        }
        output
    }

    pub fn len(&self) -> u32 {
        match &self.data {
            Some(SampleDataType::Depth8(d)) => d.len() as u32,
            Some(SampleDataType::Depth16(d)) => d.len() as u32,
            _ => 0,
        }
    }
}
