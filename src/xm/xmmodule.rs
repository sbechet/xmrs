use serde::{Serialize, Deserialize};
use bincode::ErrorKind;


use super::xmheader::{ XmHeader, XmFlagType };
use super::xmpattern::XmPattern;
use super::xminstrument::XmInstrument;

use crate::module::{ Module, ModuleFlag };

#[derive(Serialize, Deserialize, Debug)]
pub struct XmModule {
    pub header: XmHeader,
    pub pattern_order: Vec<u8>,
    pub pattern: Vec<XmPattern>,
    pub instrument: Vec<XmInstrument>,
}

impl Default for XmModule {
    fn default() -> Self {
        XmModule {
            header: XmHeader::default(),
            pattern_order: vec![],
            pattern: vec![],
            instrument: vec![],
        }
    }
}

impl XmModule {
    pub fn load(data: &[u8]) -> Result<Self, Box<ErrorKind>> {
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
            flags: match self.header.flags {
                XmFlagType::XmAmigaFrequencies => ModuleFlag::AmigaFrequencies,
                XmFlagType::XmLinearFrequencies => ModuleFlag::LinearFrequencies,
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
        let (header, pattern_order) = XmHeader::from_module(&module);
        xmm.header = header;
        xmm.pattern_order = pattern_order.try_into().unwrap();
        xmm.pattern = XmPattern::from_module(&module);
        xmm.instrument = XmInstrument::from_module(&module);
        xmm
    }

    pub fn save(&mut self) -> Result<Vec<u8>, Box<ErrorKind>> {
        let po_len = self.pattern_order.len();
        self.header.header_size = 20 + po_len as u32;
        let mut header_ser = bincode::serialize(&self.header).unwrap();
        let mut pattern_order_ser = self.pattern_order.clone();
        let mut pattern_ser: Vec <u8> = vec![];
        for xmp in &mut self.pattern {
            let mut b = xmp.save()?;
            pattern_ser.append(&mut b);
        }
        
        let mut instr_ser: Vec <u8> = vec![];
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