use serde::{Serialize, Deserialize};
use bincode::ErrorKind;
use serde_big_array::BigArray;

use crate::module::Module;
use crate::envelope::{Envelope, EnvelopePoint};
use crate::vibrato::{ Vibrato, Waveform};
use crate::instrument::{ Instrument, InstrumentType, InstrDefault};
use crate::sample::Sample;

use super::serde_helper::{ serialize_string_22, deserialize_string_22};
use super::xmsample::XmSample;

#[derive(Serialize, Deserialize, Debug)]
pub enum XmInstrumentType {
    Empty,
    Default(XmInstrDefault),
}

impl XmInstrumentType {
    pub fn save(&self) -> Result<Vec<u8>, Box<ErrorKind>> {
        match self {
            XmInstrumentType::Default(xmid) => bincode::serialize(xmid),
            _ => Ok(vec![]),
        }
    }
}

 #[derive(Serialize, Deserialize, Debug)]
 pub struct XmInstrDefault {
    sample_header_size: u32,    // not used?
    #[serde(with = "BigArray")]
    sample_for_notes: [u8; 96],

    #[serde(with = "BigArray")]
    volume_envelope: [u8; 4*12],
    #[serde(with = "BigArray")]
    panning_envelope: [u8; 4*12],
    number_of_volume_points: u8,
    number_of_panning_points: u8,
    volume_sustain_point: u8,
    volume_loop_start_point: u8,
    volume_loop_end_point: u8,
    panning_sustain_point: u8,
    panning_loop_start_point: u8,
    panning_loop_end_point: u8,
    volume_flag: u8,
    panning_flag: u8,

    vibrato_type: u8,
    vibrato_sweep: u8,
    vibrato_depth: u8,
    vibrato_rate: u8,

    volume_fadeout: u16,

    // reserved: u16,
    midi_on: u8,
    midi_channel: u8,
    // +5
    // midi_program: u16,
    // midi_bend: u16,
    // mute: u8,

}

impl Default for XmInstrDefault {
    fn default() -> Self {
        Self {
            sample_header_size: 40,
            sample_for_notes: [0; 96],
            volume_envelope: [0; 4*12],
            panning_envelope: [0; 4*12],
            number_of_volume_points: 0,
            number_of_panning_points: 0,
            volume_sustain_point: 0,
            volume_loop_start_point: 0,
            volume_loop_end_point: 0,
            panning_sustain_point: 0,
            panning_loop_start_point: 0,
            panning_loop_end_point: 0,
            volume_flag: 0,
            panning_flag: 0,
        
            vibrato_type: 0,
            vibrato_sweep: 0,
            vibrato_depth: 0,
            vibrato_rate: 0,
        
            volume_fadeout: 0,
        
            midi_on: 0,
            midi_channel: 0,
        }
    }
}


impl XmInstrDefault {
    
    fn from_envelope(e: &Envelope) -> [u8; 48] {
        let mut dst: [u8; 48] = [0; 48];
        let mut i = 0;
        for ep in &e.point {
            let f = ep.frame.to_le_bytes();
            let v = ep.value.to_le_bytes();
            dst[i+0] = f[0];
            dst[i+1] = f[1];
            dst[i+2] = v[0];
            dst[i+3] = v[1];
            i+=4;
        }
        dst
    }

    pub fn from_instr(i: &Instrument) -> XmInstrumentType {
        let mut xmid: Self = XmInstrDefault::default();
        match &i.instr_type {
            InstrumentType::Default(id) => {
                xmid.sample_for_notes = id.sample_for_note.clone().try_into().unwrap();

                xmid.volume_envelope = Self::from_envelope(&id.volume_envelope);
                xmid.number_of_volume_points = id.volume_envelope.point.len() as u8;
                xmid.volume_sustain_point = id.volume_envelope.sustain_point;
                xmid.volume_loop_start_point = id.volume_envelope.loop_start_point;
                xmid.volume_loop_end_point = id.volume_envelope.loop_end_point;
                if id.volume_envelope.enabled {
                    xmid.volume_flag |= 0b0001;
                }
                if id.volume_envelope.sustain_enabled {
                    xmid.volume_flag |= 0b0010;
                }
                if id.volume_envelope.loop_enabled {
                    xmid.volume_flag |= 0b0100;
                }

                xmid.panning_envelope = Self::from_envelope(&id.panning_envelope);
                xmid.number_of_panning_points = id.panning_envelope.point.len() as u8;
                xmid.panning_sustain_point = id.panning_envelope.sustain_point;
                xmid.panning_loop_start_point = id.panning_envelope.loop_start_point;
                xmid.panning_loop_end_point = id.panning_envelope.loop_end_point;
                if id.panning_envelope.enabled {
                    xmid.panning_flag |= 0b0001;
                }
                if id.panning_envelope.sustain_enabled {
                    xmid.panning_flag |= 0b0010;
                }
                if id.panning_envelope.loop_enabled {
                    xmid.panning_flag |= 0b0100;
                }

                xmid.vibrato_type = id.vibrato.waveform.try_into().unwrap();
                xmid.vibrato_sweep = id.vibrato.sweep;
                xmid.vibrato_depth = id.vibrato.depth;
                xmid.vibrato_rate = id.vibrato.speed;

                xmid.volume_fadeout = id.volume_fadeout;
                XmInstrumentType::Default(xmid)  
            }
            _ => {
                XmInstrumentType::Empty
            },
        }
    }
}


const XMINSTRUMENT_SIZE: usize = 29;

#[derive(Serialize, Deserialize, Debug)]
pub struct XmInstrumentHeader {
    instrument_header_len: u32,
    #[serde(deserialize_with = "deserialize_string_22", serialize_with = "serialize_string_22")]
    name: String,
    instr_type: u8, // must be 0, be random...
    num_samples: u16,
}

impl Default for XmInstrumentHeader {
    fn default() -> Self {
        Self {
            instrument_header_len: 29,
            name: String::new(),
            instr_type: 0,
            num_samples: 0,
        }
    }
}

impl XmInstrumentHeader {

    pub fn save(&self) -> Result<Vec<u8>, Box<ErrorKind>> {
        bincode::serialize(&self)
    }

    pub fn from_instr(i: &Instrument) -> Self {
        let mut xmih = XmInstrumentHeader::default();
        xmih.name = i.name.clone();
        xmih.num_samples = match &i.instr_type {
            InstrumentType::Default(it) => {
                it.sample.len() as u16
            },
            _ => 0,
        };
        xmih
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XmInstrument {
    header: XmInstrumentHeader,
    instr: XmInstrumentType,
    sample: Vec<XmSample>,
}

impl Default for XmInstrument {
    fn default() -> Self {
        Self {
            header: XmInstrumentHeader::default(),
            instr: XmInstrumentType::Empty,
            sample: vec![],
        }
    }
}

impl XmInstrument {
    pub fn load(data: &[u8]) -> Result<(&[u8], XmInstrument), Box<ErrorKind>> {
        let mut sample: Vec<XmSample> = vec![]; 

        // xmih
        let xmih = bincode::deserialize::<XmInstrumentHeader>(data)?;
        let xmih_len = xmih.instrument_header_len as usize;

        if xmih.num_samples == 0 {
            let data = &data[xmih_len..];
            let xmi = XmInstrument {
                header: xmih,
                instr: XmInstrumentType::Empty,
                sample: vec![],
            };
            return Ok( (data, xmi) );
        }

        // samples header
        let d2 = &data[XMINSTRUMENT_SIZE..];
        let xmid = bincode::deserialize::<XmInstrDefault>(d2)?;


        let mut d3 = &data[xmih_len..];
        for _ in 0..xmih.num_samples {
            let (d, s) = XmSample::load(&d3)?;
            sample.push(s);
            d3 = d;
        }

        let xmi = XmInstrument {
            header: xmih,
            instr: XmInstrumentType::Default(xmid),
            sample: sample,
        };
        let data = d3;
        Ok( (data, xmi) )
    }

    pub fn save(&mut self) -> Result<Vec<u8>, Box<ErrorKind>> {
        let mut i = self.instr.save()?;
        let mut vs : Vec<u8> = vec![];
        for s in &mut self.sample {
            let mut b = s.save()?;
            vs.append(&mut b);
        }

        self.header.num_samples = self.sample.len() as u16;
        self.header.instrument_header_len = 29 + i.len() as u32;
        let mut h = self.header.save()?;

        let mut all: Vec<u8> = vec![];
        all.append(&mut h);
        all.append(&mut i);
        all.append(&mut vs);
        Ok(all)
    }

    fn envelope_from_slice(src: &[u8]) -> Option<Envelope> {
        let mut e = Envelope::default();
        let mut iter = src.chunks_exact(2).map(|chunk| { u16::from_le_bytes([chunk[0], chunk[1]])});
        for _i in 0..src.len()/4 {
            let ep = EnvelopePoint {
                frame: iter.next()?,
                value: iter.next()?,
            };
            e.point.push(ep);
        }
        Some(e)
    }

    fn is_envelope_nok(e: &Envelope) -> bool {
        if e.point.len() > 12 || 
            e.sustain_point >= e.point.len() as u8||
            e.loop_start_point >= e.point.len()  as u8 ||
            e.loop_end_point >= e.point.len()  as u8 ||
            e.loop_start_point > e.loop_end_point {
            return true;
        } else {
            return false;
        }
    }

    pub fn to_instrument(&self) -> Instrument {
        let it:InstrumentType = match &self.instr {
            XmInstrumentType::Empty => InstrumentType::Empty,
            XmInstrumentType::Default(xmi) => {
                let mut sample: Vec<Sample> = vec![];
                for xms in &self.sample {
                    let s = xms.to_sample();
                    sample.push(s);
                }

                let num_vol_pt = if xmi.number_of_volume_points as usize <= 12 { 4*xmi.number_of_volume_points as usize } else { 0 };
                let num_pan_pt = if xmi.number_of_panning_points as usize <= 12 { 4*xmi.number_of_panning_points as usize } else { 0 };
                let mut id = InstrDefault {
                    sample_for_note: xmi.sample_for_notes.to_vec(),
                    volume_envelope: Self::envelope_from_slice(&xmi.volume_envelope[0..num_vol_pt]).unwrap(),
                    panning_envelope: Self::envelope_from_slice(&xmi.panning_envelope[0..num_pan_pt]).unwrap(),
                    vibrato: Vibrato::default(),
                    volume_fadeout: xmi.volume_fadeout,
                    sample: sample,
                };

                // copy volume envelope data
                id.volume_envelope.enabled = xmi.volume_flag & 0b0001 != 0;
                id.volume_envelope.sustain_enabled = xmi.volume_flag & 0b0010 != 0;
                id.volume_envelope.sustain_point = xmi.volume_sustain_point;
                id.volume_envelope.loop_enabled = xmi.volume_flag & 0b0100 != 0;
                id.volume_envelope.loop_start_point = xmi.volume_loop_start_point;
                id.volume_envelope.loop_end_point = xmi.volume_loop_end_point;

                // copy panning envelope data
                id.panning_envelope.enabled = xmi.panning_flag & 0b0001 != 0;
                id.panning_envelope.sustain_enabled = xmi.panning_flag & 0b0010 != 0;
                id.panning_envelope.sustain_point = xmi.panning_sustain_point;
                id.panning_envelope.loop_enabled = xmi.panning_flag & 0b0100 != 0;
                id.panning_envelope.loop_start_point = xmi.panning_loop_start_point;
                id.panning_envelope.loop_end_point = xmi.panning_loop_end_point;

                // cleanup bad envelope
                if Self::is_envelope_nok(&id.volume_envelope) {
                    id.volume_envelope = Envelope::default();
                }
                if Self::is_envelope_nok(&id.panning_envelope) {
                    id.panning_envelope = Envelope::default();
                }

                // cleanup bad sample for notes
                let sample_qty = id.sample.len();
                for i in 0..id.sample_for_note.len() {
                    if id.sample_for_note[i] as usize >= sample_qty {
                        id.sample_for_note[i] = 0;
                    }
                }

                // vibrato
                id.vibrato.waveform = match xmi.vibrato_type {
                    0 => Waveform::Sine,
                    1 => Waveform::RampDown,
                    2 => Waveform::Square,
                    3 => Waveform::Random,
                    4 => Waveform::RampUp,
                    _ => Waveform::Sine,
                };
                id.vibrato.speed = xmi.vibrato_rate;
                id.vibrato.depth = xmi.vibrato_depth;
                id.vibrato.sweep = xmi.vibrato_sweep;

                InstrumentType::Default(id)
            },
        };
        Instrument {
            name: self.header.name.clone(),
            instr_type: it,
        }
    }


    // All instr
    pub fn from_module(module: &Module) -> Vec<Self> {
        let mut all: Vec<XmInstrument> = vec![];
        for i in &module.instrument {
            let mut xmi = XmInstrument::default();
            xmi.header = XmInstrumentHeader::from_instr(&i);
            xmi.instr = XmInstrDefault::from_instr(&i);
            xmi.sample = XmSample::from_instr(&i);
            all.push(xmi);
        }
        all
    }
}