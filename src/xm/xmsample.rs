/// Original XM Sample
use bincode::ErrorKind;
use serde::{Deserialize, Serialize};

use super::helper::*;
use super::serde_helper::{deserialize_string_22, serialize_string_22};
use crate::instrument::{Instrument, InstrumentType};
use crate::sample::{LoopType, Sample, SampleDataType};

const XMSAMPLE_HEADER_SIZE: usize = 40;

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
    data: SampleDataType,
}

impl Default for XmSample {
    fn default() -> Self {
        XmSample {
            header: XmSampleHeader::default(),
            data: SampleDataType::Depth8(vec![]),
        }
    }
}

impl XmSample {
    pub fn load(data: &[u8]) -> Result<(&[u8], XmSample), Box<ErrorKind>> {
        let sh = bincode::deserialize::<XmSampleHeader>(data)?;

        // std::mem::size_of::<XmSampleHeader>() not working because alignement?
        let d2 = &data[XMSAMPLE_HEADER_SIZE..];

        let data_len: usize = sh.length as usize;
        let slice = &d2[..data_len];

        let d3 = if sh.flags & 0b0001_0000 != 0 {
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

        // Now create XmSample

        let xms = XmSample {
            header: sh,
            data: d3,
        };

        let packet_size = XMSAMPLE_HEADER_SIZE + data_len;
        Ok((&data[packet_size..], xms))
    }

    pub fn save(&mut self) -> Result<Vec<u8>, Box<ErrorKind>> {
        self.header.length = match &self.data {
            SampleDataType::Depth8(d) => d.len() as u32,
            SampleDataType::Depth16(d) => 2 * d.len() as u32,
        };

        let mut v = match &self.data {
            SampleDataType::Depth8(d) => sample8_to_delta(d),
            SampleDataType::Depth16(d) => {
                let d = sample16_to_delta(d);
                vec_u16_to_u8_slice(d)
            }
        };

        let mut all = bincode::serialize(&self.header)?;
        all.append(&mut v);
        Ok(all)
    }

    pub fn to_sample(&self) -> Sample {
        let mut loop_start = self.header.loop_start;
        let mut loop_length = self.header.loop_length;

        if let SampleDataType::Depth16(_) = &self.data {
            loop_start >>= 1;
            loop_length >>= 1;
        }

        Sample {
            name: self.header.name.clone(),
            loop_start: loop_start,
            loop_length: loop_length,
            volume: self.header.volume as f32 / 64.0,
            finetune: self.header.finetune as f32 / 128.0,
            flags: match self.header.flags & 0b000000_11 {
                1 => LoopType::Forward,
                2 => LoopType::PingPong,
                _ => LoopType::No,
            },
            panning: self.header.panning as f32 / 255.0,
            relative_note: self.header.relative_note,
            data: self.data.clone(),
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
                xms.header.finetune = (s.finetune * 128.0) as i8;
                xms.header.flags = s.flags.into();
                xms.header.panning = (s.panning * 255.0) as u8;
                xms.header.relative_note = s.relative_note;
                xms.header.name = s.name.clone();
                xms.data = s.data.clone();
                output.push(xms);
            }
        }
        output
    }
}
