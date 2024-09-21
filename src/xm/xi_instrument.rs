use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;

use super::serde_helper::{deserialize_string_20, serialize_string_20};
use super::serde_helper::{deserialize_string_21, serialize_string_21};
use super::serde_helper::{deserialize_string_22, serialize_string_22};

use super::xminstrument::{
    XmInstrDefault, XmInstrument, XmInstrumentHeader, XmInstrumentType, XMINSTRDEFAULT_SIZE,
    XMINSTRUMENT_SIZE,
};
use super::xmsample::{XmSample, XMSAMPLE_HEADER_SIZE};

const XMINSTRUMENT_HEADER: usize = 21 + 22 + 1 + 20 + 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct XiInstrumentHeader {
    #[serde(
        deserialize_with = "deserialize_string_21",
        serialize_with = "serialize_string_21"
    )]
    id_text: String, // "Extended Instrument: "
    #[serde(
        deserialize_with = "deserialize_string_22",
        serialize_with = "serialize_string_22"
    )]
    name: String,
    right_arrow: u8, // 0x1A on IBM437 charset matches →. For `COMMAND.COM` `TYPE` command, it is Ctrl-Z or EOF. Yes: a CP/M heritage...so old!
    #[serde(
        deserialize_with = "deserialize_string_20",
        serialize_with = "serialize_string_20"
    )]
    tracker_name: String, // example: "Fasttracker II clone"
    version_number: u16,
}

impl Default for XiInstrumentHeader {
    fn default() -> Self {
        Self {
            id_text: "Extended Instrument: ".to_string(),
            name: "".to_string(),
            right_arrow: 0x1A,
            tracker_name: "XMrs".to_string(), // or "Fasttracker II clone"
            version_number: 0x0104,           // minimal version number supported
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XiInstrument {
    header: XiInstrumentHeader,
    instr: XmInstrDefault,
    reserved: [u8; 15],
    num_samples: u16,
}

impl XiInstrument {
    pub fn load(data: &[u8]) -> Result<XmInstrument, DecodeError> {
        let xi =
            bincode::serde::decode_from_slice::<XiInstrument, _>(data, bincode::config::legacy())?
                .0;
        let seek = XMINSTRUMENT_HEADER + XMINSTRDEFAULT_SIZE + 15 + 2;
        let data = &data[seek..];

        if xi.header.id_text != "Extended Instrument:" {
            return Err(DecodeError::Other("Not an Extended Instrument?"));
        }

        //---

        let header = XmInstrumentHeader {
            instrument_header_len: XMINSTRUMENT_SIZE as u32,
            name: xi.header.name,
            instr_type: 0,
            num_samples: xi.num_samples,
        };

        if xi.num_samples == 0 {
            let xmi = XmInstrument {
                header: header,
                sample_header_size: XMSAMPLE_HEADER_SIZE as u32,
                instr: XmInstrumentType::Default(Box::new(xi.instr)),
                sample: vec![],
            };
            return Ok(xmi);
        }

        // all samples headers, then data...
        let mut sample = vec![];

        let mut d3 = data;
        for _ in 0..xi.num_samples {
            let (d, s) = XmSample::load(d3)?;
            sample.push(s);
            d3 = d;
        }

        for s in &mut sample {
            let d = s.add_sample(d3)?;
            d3 = d;
        }

        let xmi = XmInstrument {
            header: header,
            sample_header_size: XMSAMPLE_HEADER_SIZE as u32,
            instr: XmInstrumentType::Default(Box::new(xi.instr)),
            sample: sample,
        };

        return Ok(xmi);
    }
}
