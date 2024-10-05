use super::serde_helper::{deserialize_string_12, deserialize_string_28, deserialize_string_4};
use bincode;
use bincode::error::DecodeError;
use serde::Deserialize;

use crate::prelude::*;

use alloc::string::String;
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

use crate::s3m::s3m_effect::S3mEffect;

#[repr(C)]
#[derive(Default, Deserialize, Debug)]
struct S3mHeader {
    #[serde(deserialize_with = "deserialize_string_28")]
    title: String,
    /// 0x1A
    sig1: u8,
    song_type: u8,
    reserved1: u16,
    order_count: u16,
    instrument_count: u16,
    pattern_count: u16,
    flags: u16,
    version: u16,
    sample_type: u16,
    /// SCRM
    #[serde(deserialize_with = "deserialize_string_4")]
    sig2: String,
    global_volume: u8,
    speed: u8,
    tempo: u8,
    master_volume: u8, // bit7: stereo, bits 6-0: volume
    ultra_click_removal: u8,
    pan: u8,
    reserved2: [u8; 8],
    ptr_special: u16, // parapointer to additional data ? see flags, bit7
    channel_settings: [u8; 32],
}

#[repr(C)]
#[derive(Deserialize, Debug, Default)]
struct S3mPcmInstr {
    /// offset = ((ptr_data_h << 16) | ptr_data_l) * 16
    ptr_data_h: u8,
    ptr_data_l: u16,

    len: u32,
    loop_start: u32,
    loop_end: u32,

    /// 0-63
    volume: u8,
    dsk: u8,
    /// 0=unpacked, 1=DP30ADPCM
    pack: u8,
    // 1=loop on, 2=stereo (data left then data right), 4=s16le
    flags: u8,
    /// sample rate for middle-c note (C-4)
    c2spd: u32,
    internal: [u8; 12],
    #[serde(deserialize_with = "deserialize_string_28")]
    title: String,
    /// SCRS
    #[serde(deserialize_with = "deserialize_string_4")]
    sig: String,
}

impl S3mPcmInstr {
    fn is_loop(&self) -> bool {
        self.flags & 1 != 0
    }

    fn is_stereo(&self) -> bool {
        self.flags & 2 != 0
    }

    fn is_16bits(&self) -> bool {
        self.flags & 4 != 0
    }

    fn get_sample_offset(&self) -> usize {
        (((self.ptr_data_h as usize) << 16) | (self.ptr_data_l as usize)) << 4
    }

    fn get_sample_data(&self, data: &[u8]) -> Result<SampleDataType, DecodeError> {
        let offset = self.get_sample_offset();
        let len = if self.get_sample_offset() + self.len as usize > data.len() {
            // fixes "miracle man.s3m" and other broken S3Ms
            data.len() - offset
        } else {
            self.len as usize
        };

        // TODO: One day, create a real xm instrument with pan for stereo samples
        // today code create mono from stereo samples.
        let dst = if self.is_16bits() {
            let src = Self::convert_u8_to_u16_vec(&data[offset..(offset + len * 2)])?;
            SampleDataType::Depth16(self.convert_16bit_sample(src.as_slice()))
        } else {
            let src = &data[offset..offset + len];
            SampleDataType::Depth8(self.convert_8bit_sample(src))
        };

        Ok(dst)
    }

    fn convert_u8_to_u16_vec(input: &[u8]) -> Result<Vec<u16>, DecodeError> {
        if input.len() % 2 != 0 {
            return Err(DecodeError::Other("input is odd!"));
        }

        let mut output = Vec::with_capacity(input.len() / 2);

        for chunk in input.chunks_exact(2) {
            let value = u16::from_le_bytes([chunk[0], chunk[1]]);
            output.push(value);
        }
        Ok(output)
    }

    fn convert_8bit_sample(&self, p: &[u8]) -> Vec<i8> {
        let length = p.len();
        let mut dst: Vec<i8> = vec![];

        if self.is_stereo() {
            let half_length = length / 2;
            let (left_channel, right_channel) = p.split_at(half_length);

            for i in 0..half_length {
                let l = left_channel[i] ^ 0x80;
                let r = right_channel[i] ^ 0x80;
                dst.push(((l as i16 + r as i16) / 2) as i8);
            }
        } else {
            for i in 0..length {
                dst.push((p[i] ^ 0x80) as i8)
            }
        }
        dst
    }

    fn convert_16bit_sample(&self, p: &[u16]) -> Vec<i16> {
        let length = p.len();
        let mut dst: Vec<i16> = vec![];

        if self.is_stereo() {
            let half_length = length / 2;
            let (left_channel, right_channel) = p.split_at(half_length);

            for i in 0..half_length {
                let l = left_channel[i] ^ 0x8000;
                let r = right_channel[i] ^ 0x8000;
                dst.push((l as i16 + r as i16) / 2);
            }
        } else {
            for i in 0..length {
                dst.push((p[i] ^ 0x8000) as i16)
            }
        }
        dst
    }
}

#[repr(C)]
#[derive(Deserialize, Debug)]
struct S3mOplInstr {
    reserved1: [u8; 3],

    mod0: u8,
    car1: u8,
    mod2: u8,
    car3: u8,
    mod4: u8,
    car5: u8,
    mod6: u8,
    car7: u8,
    mod8: u8,
    car9: u8,
    mod10: u8,
    unused11: u8,

    volume: u8,
    dsk: u8,
    reserved2: u16,
    c2spd: u32,
    internal: [u8; 12],
    #[serde(deserialize_with = "deserialize_string_28")]
    pub title: String,
    /// SCRI
    #[serde(deserialize_with = "deserialize_string_4")]
    sig: String,
}

impl S3mOplInstr {
    // TODO: check why self.mod0#tremolo and self.mod0#sustain not used.
    pub fn to_instr_opl(&self) -> InstrOpl {
        let ph = PeriodHelper::new(FrequencyType::LinearFrequencies, false);
        let mut i_opl = InstrOpl::default();
        i_opl.element.modulator = MdiOpl {
            ksl: ((self.mod2 & 0b0100_0000) >> 6) | (self.mod2 >> 7),
            multiple: self.mod0 & 0b0000_1111,
            feedback: self.mod10 >> 1,
            attack: self.mod4 >> 4,
            sustain: self.mod6 >> 4,
            eg: (self.mod0 & 0b0010_0000) != 0, // FIXME: Sustain?
            decay: self.mod4 & 0b0000_1111,
            release: self.mod6 & 0b0000_1111,
            total_level: self.mod2 & 0b0011_1111,
            am: (self.mod0 & 0b1000_0000) != 0, // FIXME: Tremolo?
            vib: (self.mod0 & 0b0100_0000) != 0,
            ksr: (self.mod0 & 0b0001_0000) != 0,
            con: (self.mod10 & 0b0000_0001) != 0,
        };
        i_opl.element.carrier = MdiOpl {
            ksl: ((self.car3 & 0b0100_0000) >> 6) | (self.car3 >> 7),
            multiple: self.car1 & 0b0000_1111,
            feedback: self.mod10 >> 1, // no way for carrier
            attack: self.car5 >> 4,
            sustain: self.car7 >> 4,
            eg: (self.car1 & 0b0010_0000) != 0, // FIXME: Sustain?
            decay: self.car5 & 0b0000_1111,
            release: self.car7 & 0b0000_1111,
            total_level: self.car3 & 0b0011_1111,
            am: (self.car1 & 0b1000_0000) != 0, // FIXME: Tremolo?
            vib: (self.car1 & 0b0100_0000) != 0,
            ksr: (self.car1 & 0b0001_0000) != 0,
            con: (self.mod10 & 0b0000_0001) != 0, // no way for carrier
        };
        i_opl.element.modulator_wave_select = self.mod8;
        i_opl.element.carrier_wave_select = self.car9;
        i_opl.volume = self.volume;
        let rn = ph.c4freq_to_relative_note(self.c2spd as f32);
        i_opl.finetune = rn.1;
        i_opl.relative_note = rn.0;

        return i_opl;
    }
}

#[derive(Deserialize, Debug)]
enum S3mInstrument {
    PcmInstrument(S3mPcmInstr),
    OplInstrument(S3mOplInstr),
}

#[repr(C)]
#[derive(Deserialize, Debug)]
struct S3mMetaInstrument {
    discriminator: u8,
    #[serde(deserialize_with = "deserialize_string_12")]
    filename: String,
    value: S3mInstrument,
    sample: Option<SampleDataType>,
}

impl S3mMetaInstrument {
    fn new(data: &[u8], offset: usize) -> Result<Self, DecodeError> {
        let discriminator = data[offset];
        let filename = String::from_utf8_lossy(&data[offset + 1..offset + 13])
            .trim_end_matches('\0')
            .to_string();
        let mut sample = None;
        let value = match discriminator {
            0 => {
                // Empty Instrument, we use PcmInstrument not to forget informations
                let i = bincode::serde::decode_from_slice::<S3mPcmInstr, _>(
                    &data[offset + 13..],
                    bincode::config::legacy(),
                )?
                .0;
                S3mInstrument::PcmInstrument(i)
            }
            1 => {
                let i = bincode::serde::decode_from_slice::<S3mPcmInstr, _>(
                    &data[offset + 13..],
                    bincode::config::legacy(),
                )?
                .0;
                if i.sig != "SCRS" {
                    return Err(DecodeError::Other("Not a SCRS instr?"));
                }
                sample = Some(i.get_sample_data(data)?);
                S3mInstrument::PcmInstrument(i)
            }
            2 | 3 | 4 | 5 | 6 | 7 => {
                let i = bincode::serde::decode_from_slice::<S3mOplInstr, _>(
                    &data[offset + 13..],
                    bincode::config::legacy(),
                )?
                .0;
                if i.sig != "SCRI" {
                    return Err(DecodeError::Other("Not a SCRI instr?"));
                }
                S3mInstrument::OplInstrument(i)
            }
            _ => S3mInstrument::PcmInstrument(S3mPcmInstr::default()),
        };
        Ok(Self {
            discriminator,
            filename,
            value,
            sample,
        })
    }
}

#[derive(Default, Deserialize, Debug)]
pub struct S3mModule {
    header: S3mHeader,
    positions: Vec<u8>,
    instruments: Vec<S3mMetaInstrument>,
    patterns: Vec<Pattern>,
}

impl S3mModule {
    pub fn load(ser_s3m_module: &[u8]) -> Result<S3mModule, DecodeError> {
        let mut s3m = S3mModule {
            ..Default::default()
        };

        // === load header

        let s = 96;
        if ser_s3m_module.len() < s {
            return Result::Err(DecodeError::Other("Not an S3M module?"));
        }
        let data = &ser_s3m_module[0..s];
        s3m.header =
            bincode::serde::decode_from_slice::<S3mHeader, _>(data, bincode::config::legacy())?.0;
        let data = &ser_s3m_module[s..];

        if s3m.header.sig1 != 0x1A || s3m.header.song_type != 0x10 || s3m.header.sig2 != "SCRM" {
            return Result::Err(DecodeError::Other("Not an S3M module?"));
        }

        // === positions offsets

        let s = s3m.header.order_count as usize;
        if data.len() < s {
            return Result::Err(DecodeError::Other("Not an S3M module?"));
        }
        s3m.positions = data[0..s].to_vec();
        let data = &data[s..];

        // remove pattern separators (254)
        s3m.positions.retain(|&x| x != 254);
        // cut on first eop (255)
        s3m.positions = s3m
            .positions
            .iter()
            .take_while(|&x| *x != 255)
            .cloned()
            .collect();

        // === sample offsets

        let s = 2 * s3m.header.instrument_count as usize;
        if data.len() < s {
            return Result::Err(DecodeError::Other("Not an S3M module?"));
        }
        let sample_offsets: Vec<u32> = (&data[0..s])
            .chunks(2)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], 0, 0]) << 4)
            .collect();
        let data = &data[s..];

        // === pattern offsets

        let s = 2 * s3m.header.pattern_count as usize;
        if data.len() < s {
            return Result::Err(DecodeError::Other("Not an S3M module?"));
        }
        let pattern_offsets: Vec<u32> = (&data[0..s])
            .chunks(2)
            .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], 0, 0]) << 4)
            .collect();

        // === Samples

        for offset in sample_offsets {
            if offset == 0 {
                continue;
            }
            // let data = &ser_s3m_module[offset as usize..];
            s3m.instruments
                .push(S3mMetaInstrument::new(ser_s3m_module, offset as usize)?);
        }

        // === Patterns

        for offset in pattern_offsets {
            if offset == 0 {
                continue;
            }
            let data = &ser_s3m_module[offset as usize..];
            let len: u16 = data[0] as u16 | (data[1] as u16) << 8;
            let data = &data[2..len as usize];
            let mut d2 = data;
            let mut pattern: Vec<Vec<PatternSlot>> = vec![];
            while d2.len() != 0 {
                let (pss, next) = Self::process_pattern_row(d2)?;
                pattern.push(pss);
                d2 = next;
            }
            S3mEffect::update_pattern(&mut pattern);
            s3m.patterns.push(pattern);
        }

        Ok(s3m)
    }

    // load one pattern row
    fn process_pattern_row(data: &[u8]) -> Result<(Vec<PatternSlot>, &[u8]), DecodeError> {
        let mut d2 = data;
        let mut pss: Vec<PatternSlot> = vec![PatternSlot::default(); 32];
        while d2.len() != 0 {
            match Self::decode_pattern_slot(d2) {
                Some((channel, ps, next)) => {
                    pss[channel] = ps;
                    d2 = next;
                }
                None => {
                    d2 = &d2[1..];
                    break;
                }
            }
        }
        return Ok((pss, d2));
    }

    // return channel, PatternSlot, next data
    fn decode_pattern_slot(packed_data: &[u8]) -> Option<(usize, PatternSlot, &[u8])> {
        let mut k = 0;

        let what = packed_data[k];
        k += 1;

        if what == 0 {
            return None; // EOL
        }

        let channel = (what & 0x1F) as usize;

        let mut slot: PatternSlot = PatternSlot::default();

        // Note+Instr?
        if what & 0x20 != 0 {
            if k < packed_data.len() {
                let note = packed_data[k];
                slot.note = match note {
                    254 => Note::KeyOff,
                    255 => Note::None,
                    _ => {
                        let tmp_note = 1 + (note & 0xF) + (note >> 4) * 12;
                        if tmp_note > 96 {
                            Note::None
                        } else {
                            Note::try_from(tmp_note).unwrap()
                        }
                    }
                };
                k += 1;
            }

            if k < packed_data.len() {
                slot.instrument = packed_data[k];
                k += 1;
            }
        }

        // Volume?
        if what & 0x40 != 0 {
            if k < packed_data.len() {
                slot.volume = packed_data[k];
                // FIXME: we ignore 255 as default instrument volume
                if slot.volume > 64 {
                    slot.volume = 64;
                }
                k += 1;
            }
        }

        // effect and data?
        if what & 0x80 != 0 {
            if k < packed_data.len() {
                slot.effect_type = packed_data[k];
                k += 1;
            }

            if k < packed_data.len() {
                slot.effect_parameter = packed_data[k];
                k += 1;
            }
        }

        Some((channel, slot, &packed_data[k..]))
    }

    pub fn to_module(&self) -> Module {
        let ph = PeriodHelper::new(FrequencyType::LinearFrequencies, false);
        let mut module = Module::default();

        module.name = self.header.title.clone();
        module.comment = "XmRs reader".to_string();
        module.frequency_type = FrequencyType::LinearFrequencies;
        module.default_tempo = self.header.speed as u16;
        module.default_bpm = self.header.tempo as u16;
        module.pattern_order = self.positions.iter().map(|&x| x as usize).collect();
        module.pattern = self.patterns.clone();

        for s3m_meta_instr in &self.instruments {
            match &s3m_meta_instr.value {
                S3mInstrument::PcmInstrument(pcm) => {
                    // Prepare sample
                    let data = if let Some(sdt) = &s3m_meta_instr.sample {
                        sdt.clone()
                    } else {
                        if pcm.is_16bits() {
                            SampleDataType::Depth16(vec![])
                        } else {
                            SampleDataType::Depth8(vec![])
                        }
                    };
                    let rn = ph.c4freq_to_relative_note(pcm.c2spd as f32);
                    let sample = Sample {
                        name: s3m_meta_instr.filename.clone(),
                        loop_start: pcm.loop_start,
                        loop_length: pcm.loop_end - pcm.loop_start,
                        volume: pcm.volume as f32 / 64.0,
                        finetune: rn.1,
                        flags: if pcm.is_loop() {
                            LoopType::Forward
                        } else {
                            LoopType::No
                        },
                        panning: 0.5,
                        relative_note: rn.0,
                        data: data,
                    };

                    // Create InstrDefault
                    let mut instr_def = InstrDefault::default();
                    instr_def.sample.push(sample);

                    // Create Instrument
                    let mut instr = Instrument::default();
                    instr.name = pcm.title.clone();
                    instr.instr_type = InstrumentType::Default(instr_def);

                    // Add Instrument to module
                    module.instrument.push(instr);
                }
                S3mInstrument::OplInstrument(opl) => {
                    // Create Instrument
                    let mut instr = Instrument::default();
                    instr.name = opl.title.clone();
                    instr.instr_type = InstrumentType::Opl(opl.to_instr_opl());

                    // Add Instrument to module
                    module.instrument.push(instr);
                }
            }
        }

        module
    }
}
