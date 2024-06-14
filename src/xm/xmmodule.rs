/// Original XM Module
use bincode::error::{DecodeError, EncodeError};
use serde::{Deserialize, Serialize};

use alloc::format;
use alloc::{vec, vec::Vec};

use super::xmheader::{XmFlagType, XmHeader};
use super::xminstrument::XmInstrument;
use super::xmpattern::XmPattern;

use crate::module::{FrequencyType, Module};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct XmModule {
    pub header: XmHeader,
    pub pattern_order: Vec<u8>,
    pub pattern: Vec<XmPattern>,
    pub instrument: Vec<XmInstrument>,
}

impl XmModule {
    pub fn load(data: &[u8]) -> Result<Self, DecodeError> {
        let (data, header, pattern_order) = XmHeader::load(data)?;
        let mut data = data;

        // Create patterns from xm
        let mut pattern: Vec<XmPattern> = vec![];
        for _i in 0..header.number_of_patterns {
            let (d2, xmp) = XmPattern::load(data, header.number_of_channels)?;
            data = d2;
            pattern.push(xmp);
        }

        let mut instrument: Vec<XmInstrument> = vec![];
        for _i in 0..header.number_of_instruments {
            // Create instruments form xm
            let (d2, xmi) = XmInstrument::load(data)?;
            data = d2;
            instrument.push(xmi);
        }

        Ok(XmModule {
            header,
            pattern_order,
            pattern,
            instrument,
        })
    }

    pub fn to_module(&self) -> Module {
        // Create module from xm
        let mut module = Module {
            name: self.header.name.clone(),
            comment: format!(
                "{} ({}.{:02})",
                self.header.tracker_name,
                self.header.version_number >> 8,
                self.header.version_number & 0xFF
            ),
            frequency_type: match self.header.flags {
                XmFlagType::XmAmigaFrequencies => FrequencyType::AmigaFrequencies,
                XmFlagType::XmLinearFrequencies => FrequencyType::LinearFrequencies,
            },
            restart_position: self.header.restart_position,
            default_tempo: self.header.default_tempo,
            default_bpm: self.header.default_bpm,
            pattern_order: self.pattern_order.clone(),
            pattern: vec![],
            instrument: vec![],
        };

        for p in &self.pattern {
            module.pattern.push(p.pattern.clone());
        }

        for i in &self.instrument {
            module.instrument.push(i.to_instrument())
        }

        module
    }

    pub fn from_module(module: &Module) -> Self {
        // Create XmModule from Module
        let mut xmm = XmModule::default();
        let (header, pattern_order) = XmHeader::from_module(module);
        xmm.header = header;
        xmm.pattern_order = pattern_order;
        xmm.pattern = XmPattern::from_module(module);
        xmm.instrument = XmInstrument::from_module(module);
        xmm
    }

    pub fn save(&mut self) -> Result<Vec<u8>, EncodeError> {
        let po_len = self.pattern_order.len();
        self.header.header_size = 20 + po_len as u32;
        let mut header_ser = bincode::serde::encode_to_vec(&self.header, bincode::config::legacy()).unwrap();
        let mut pattern_order_ser = self.pattern_order.clone();
        let mut pattern_ser: Vec<u8> = vec![];
        for xmp in &mut self.pattern {
            let mut b = xmp.save()?;
            pattern_ser.append(&mut b);
        }

        let mut instr_ser: Vec<u8> = vec![];
        for xmi in &mut self.instrument {
            let mut b = xmi.save()?;
            instr_ser.append(&mut b);
        }

        let mut all: Vec<u8> = vec![];
        all.append(&mut header_ser);
        all.append(&mut pattern_order_ser);
        all.append(&mut pattern_ser);
        all.append(&mut instr_ser);
        Ok(all)
    }
}
