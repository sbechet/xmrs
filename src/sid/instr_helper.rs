use crate::{instr_sid::SidVoice, prelude::*};
use alloc::format;
use alloc::string::String;
use alloc::{vec, vec::Vec};

#[cfg(feature = "micromath")]
use micromath::F32Ext;
#[cfg(feature = "libm")]
use num_traits::float::Float;

pub struct InstrHelper;

impl InstrHelper {
    fn sid_to_sample(voice: &SidVoice) -> Sample {
        let mut name = String::from("Unknown Waveform");
        let mut data = SampleDataType::Depth8(vec![0; 128]); // Par défaut: silence
        let flags = LoopType::Forward;

        // Définir les paramètres basés sur les contrôles de SidVoice
        if voice.ctrl_noise {
            name = String::from("Noise");
            data = Self::generate_sid_noise_sample(); // Génération d'un sample bruité
        } else if voice.ctrl_pulse {
            name = format!("Pulse Wave {}", voice.pw);
            data = Self::generate_pulse_sample(voice.pw); // Génération d'un sample onde carrée avec largeur d'impulsion
                                                          // flags = LoopType::Forward;  // Peut-être que les ondes carrées peuvent boucler en avant
        } else if voice.ctrl_sawtooth {
            name = String::from("Sawtooth Wave");
            data = Self::generate_sawtooth_sample(); // Génération d'un sample onde dent de scie
                                                     // finetune = 0.25;  // Une dent de scie peut avoir un finetune particulier
        } else if voice.ctrl_triangle {
            name = String::from("Triangle Wave");
            data = Self::generate_triangle_sample(); // Génération d'un sample onde triangle
                                                     // flags = LoopType::PingPong;  // Exemple: onde triangle en ping-pong
        }

        // Définir le sample
        Sample {
            name,
            loop_start: 0,
            loop_length: 128, // Exemple arbitraire: longueur de boucle
            volume: 1.0,      // Volume par défaut
            finetune: 0.0,
            flags,
            panning: 0.5,
            relative_note: 0, // Correspond à C-4 par défaut
            data,
        }
    }

    fn generate_sid_noise_sample() -> SampleDataType {
        let mut lfsr: u16 = 0xACE1;
        let mut noise_data: Vec<i8> = vec![0; 128];

        for i in 0..128 {
            let new_bit = (lfsr >> 0) ^ (lfsr >> 1) & 1;
            lfsr = (lfsr >> 1) | (new_bit << 15);
            let value_u8 = (lfsr & 0xFF) as u8;
            noise_data[i] = value_u8.wrapping_sub(128) as i8;
        }

        SampleDataType::Depth8(noise_data)
    }

    fn generate_pulse_sample(pw: u16) -> SampleDataType {
        let mut pulse_data: Vec<i8> = vec![0; 128];
        let duty_cycle = (pw as f32 / 4095.0 * 128.0) as usize;
        for i in 0..128 {
            pulse_data[i] = if i < duty_cycle { 127 } else { -127 };
        }
        SampleDataType::Depth8(pulse_data)
    }

    fn generate_sawtooth_sample() -> SampleDataType {
        let sawtooth_data: Vec<i8> = (0..128)
            .map(|x| (2 * x as u8).wrapping_sub(128) as i8)
            .collect();
        SampleDataType::Depth8(sawtooth_data)
    }

    fn generate_triangle_sample() -> SampleDataType {
        let mut triangle_data: Vec<i8> = vec![0; 128];

        for i in 0..128 {
            let value = if i < 64 {
                ((4 * i) as u8).wrapping_sub(128) as i8
            } else {
                ((4 * (127 - i)) as u8).wrapping_sub(128) as i8
            };

            triangle_data[i as usize] = value;
        }

        SampleDataType::Depth8(triangle_data)
    }

    fn gen_adr_frames(bpm: u16, speed: u8) -> [u16; 16] {
        let tick_duration_ms = (2500.0 / bpm as f32) * speed as f32;

        let adr_durations_ms: [u32; 16] = [
            2, 8, 16, 24, 38, 56, 68, 80, 100, 250, 500, 800, 1100, 1500, 2400, 3900,
        ];

        let mut frames_for_adr: [u16; 16] = [0; 16];
        for (i, &duration_ms) in adr_durations_ms.iter().enumerate() {
            frames_for_adr[i] = (duration_ms as f32 / tick_duration_ms).round() as u16;
        }

        frames_for_adr
    }

    fn sid_to_xi(sid: &SidVoice) -> InstrDefault {
        // let adr_frames = Self::gen_adr_frames(125, 2);
        let adr_frames: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        // ADSR
        let attack = (sid.ad >> 4) & 0x0F;
        let decay = sid.ad & 0x0F;
        let sustain = (sid.sr >> 4) & 0x0F;
        let release = sid.sr & 0x0F;

        let mut point: Vec<EnvelopePoint> = vec![];
        let mut seek: usize = 0;
        let mut seek_point = 0;

        if attack != 0 {
            let attack_frame = adr_frames[attack as usize];
            point.push(EnvelopePoint { frame: 0, value: 0.0 });
            seek += attack_frame;
            point.push(EnvelopePoint {
                frame: attack_frame,
                value: 1.0,
            });
            seek_point = 1;
        } else {
            point.push(EnvelopePoint {
                frame: 0,
                value: 1.0,
            });
        }

        if decay != 0 {
            seek += adr_frames[decay as usize];
        } else {
            seek += 1;
        }
        point.push(EnvelopePoint {
            frame: seek,
            value: 4.0 * sustain as f32 / 64.0,
        });
        seek_point += 1;

        if release != 0 {
            seek += adr_frames[release as usize];
        } else {
            seek += 1;
        }
        point.push(EnvelopePoint {
            frame: seek,
            value: 0.0,
        });

        let volume_envelope = Envelope {
            enabled: true,
            point,
            sustain_enabled: sustain != 0,
            sustain_point: seek_point,
            loop_enabled: false,
            loop_start_point: 0,
            loop_end_point: 0,
        };

        let mut instr = InstrDefault::default();
        instr.volume_envelope = volume_envelope;
        //FIXME: instr.volume_fadeout = 1.0;

        //FIXME: instr.sample = ...;
        let mut s: Sample = Self::sid_to_sample(&sid);
        s.relative_note = 24;
        instr.sample = vec![s];

        instr
    }

    fn irs_to_instrument(irs: &InstrRobSid, original: bool) -> Instrument {
        let mut idst = Instrument::default();
        if irs.sid.voice[0].ctrl_noise {
            idst.name = format!(
                "Noise {:02X}{:02X}",
                irs.sid.voice[0].ad, irs.sid.voice[0].sr
            );
        //            idst.name = String::from("Noise");
        } else if irs.sid.voice[0].ctrl_pulse {
            idst.name = format!(
                "Pulse {:02X}{:02X}",
                irs.sid.voice[0].ad, irs.sid.voice[0].sr
            );
        //            idst.name = String::from("Pulse");
        } else if irs.sid.voice[0].ctrl_sawtooth {
            idst.name = format!(
                "Sawtooth {:02X}{:02X}",
                irs.sid.voice[0].ad, irs.sid.voice[0].sr
            );
            // idst.name = String::from("Sawtooth");
        } else if irs.sid.voice[0].ctrl_triangle {
            idst.name = format!(
                "Triangle {:02X}{:02X}",
                irs.sid.voice[0].ad, irs.sid.voice[0].sr
            );
            // idst.name = String::from("Triangle");
        }
        if original {
            idst.instr_type = InstrumentType::RobSid(irs.clone());
        } else {
            let mut xi = Self::sid_to_xi(&irs.sid.voice[0]);
            if irs.fx[0].vibrato {
                xi.vibrato.depth = irs.fx[0].vibrato_depth as f32 / 15.0;
                xi.vibrato.speed = 1.0 / irs.fx[0].vibrato_div as f32;
            }
            idst.instr_type = InstrumentType::Default(xi);
        }
        idst
    }

    pub fn irss_to_instruments(irss: &Vec<InstrRobSid>, original: bool) -> Vec<Instrument> {
        let mut idst: Vec<Instrument> = vec![];
        for isrc in irss {
            idst.push(Self::irs_to_instrument(isrc, original));
        }
        idst
    }
}
