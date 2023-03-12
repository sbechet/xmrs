use serde::{Serialize, Deserialize};
use bincode::ErrorKind;

use crate::module::Module;

use super::xmpatternslot::XmPatternSlot;

#[derive(Serialize, Deserialize, Debug)]
pub struct XmPatternHeader {
    pattern_header_len: u32,
    packing_type: u8,
    num_rows: u16,
    pattern_data_size: u16,
}

impl Default for XmPatternHeader {
    fn default() -> Self {
        XmPatternHeader {
            pattern_header_len: 9,
            packing_type: 0,
            num_rows: 0,
            pattern_data_size: 0,        
        }
    }
}

impl XmPatternHeader {
    pub fn load(data: &[u8]) -> Result<(&[u8], XmPatternHeader), Box<ErrorKind>> {
        match bincode::deserialize::<XmPatternHeader>(data) {
            Ok(xmph) => {
                let hl = xmph.pattern_header_len as usize;
                Ok( (&data[hl..], xmph) )
            },
            Err(e) => Err(e),
        }
    }
}

type Lines = Vec<Vec<XmPatternSlot>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct XmPattern {
    pub header: XmPatternHeader,
    pub pattern: Lines,
}

impl Default for XmPattern {
    fn default() -> Self {
        Self {
            header: XmPatternHeader::default(),
            pattern: vec![],
        }
    }
}

impl XmPattern {
    pub fn load(data: &[u8], number_of_channels: u16) -> Result<(&[u8], XmPattern), Box<ErrorKind>> {
        let (data, xmph) = XmPatternHeader::load(data)?;
        let (_, xmps) = Self::get_slots(&data[0..xmph.pattern_data_size as usize], number_of_channels).unwrap();
        let seek = xmph.pattern_data_size as usize;

        let xmp = XmPattern {
            header: xmph,
            pattern: xmps,
        };

        Ok( (&data[seek..], xmp) )
    }

    fn get_slots<'a>(data: &'a[u8], number_of_channels: u16) -> Result<(&'a[u8], Vec<Vec<XmPatternSlot>>), Box<ErrorKind>> {
        let mut lines: Vec<Vec<XmPatternSlot>> = vec![];
        let mut row: Vec<XmPatternSlot> = vec![];

        let mut d2 = data;
        loop {
            if d2.len() == 0 {
                break;
            }
            let (d3, xps) = XmPatternSlot::load(d2)?;
            d2 = d3;
            row.push(xps);
            if row.len() == number_of_channels as usize {
                lines.push(row);
                row = vec![];
            }    
        }

        Ok( (data, lines) )
    }

    /// All patterns
    pub fn from_module(module: &Module) -> Vec<Self> {
        let mut all: Vec<Self> = vec![];
        for p in &module.pattern {
            let mut xmp = Self::default();
            xmp.pattern = p.clone();
            xmp.header.num_rows = p.len() as u16;
            // uncompressed patternslot
            xmp.header.pattern_data_size = (p.len() * p[0].len() * 5) as u16;
            all.push(xmp);
        }
        all
    }

    pub fn save(&mut self) -> Result<Vec<u8>, Box<ErrorKind>> {
        let mut p_output: Vec<u8> = vec![];

        for p in &self.pattern {
            for ps in p {
                let mut b = ps.save();
                p_output.append(&mut b);
            }
        }
        self.header.pattern_data_size = p_output.len() as u16;

        let mut output = bincode::serialize(&self.header)?;
        output.append(&mut p_output);
        Ok(output)
    }

}






