use crate::{instr_robsid::RobEffects, instr_sid::SidVoice, prelude::*};
use alloc::{vec, vec::Vec};

use super::pattern_helper::PatternHelper;
use super::sid_module::SidModule;
use super::sound_fx::SoundFx;

#[derive(Clone, Debug)]
pub struct OneSid {
    pub song: &'static [u8],
    pub name: &'static str,
    pub author: &'static str,
    pub copyright: &'static str,
    version: usize,
    load_adress: usize,
    song_track_qty: usize,
    song_list_offset: usize,
    song_list_qty: usize,
    patt_ptl_offset: usize,
    patt_pth_offset: usize,
    patt_qty: usize,
    instr_offset: usize,
    instr_qty: usize,
    fx_v1_offset: usize,
    fx_v1_qty: usize,
    fx_v2_offset: usize,
    fx_v2_qty: usize,
    pub resetspd: usize,
    skydive_v1_when: usize, // When length > skydive_v1_when
    skydive_v1_add: isize,  // XXX if freq+skydive_v1_add<0x10000
}

impl OneSid {
    pub fn to_sidmodule(&self) -> SidModule {
        // ********* CHANNELS
        let mut channels_ptr: Vec<usize> = vec![];
        for i in 0..self.song_list_qty {
            let offset =
                126 + self.song_list_offset - self.load_adress + i * 2 * self.song_track_qty;
            for j in 0..self.song_track_qty {
                channels_ptr.push(
                    self.song[offset + j] as usize
                        | (self.song[offset + j + self.song_track_qty] as usize) << 8,
                );
            }
        }

        let mut channels: Vec<Vec<u8>> = vec![];
        for offset in &channels_ptr {
            let file_offset = *offset as usize + 126 - self.load_adress;
            let mut tracks: Vec<u8> = vec![];
            let mut j = 0;
            while self.song[file_offset + j] & 0x80 == 0 {
                tracks.push(self.song[file_offset + j]);
                j += 1;
            }
            channels.push(tracks);
        }

        // ********* SONGS
        let mut songs: Vec<Vec<usize>> = vec![];
        for i in 0..self.song_list_qty {
            let mut s: Vec<usize> = vec![];
            for j in 0..self.song_track_qty {
                //let k = channels_ptr[i * self.song_track_qty + j] + 126 - self.load_adress;
                let k = channels_ptr[i * self.song_track_qty + j];
                let index = match channels_ptr.iter().position(|&x| x == k) {
                    Some(index) => index,
                    None => 0,
                };
                s.push(index);
            }
            songs.push(s);
        }

        // ********* TRACKS
        let mut tracks_ptr: Vec<u16> = vec![];
        for i in 0..self.patt_qty {
            let offset_low_file = 126 + self.patt_ptl_offset - self.load_adress;
            let offset_high_file = 126 + self.patt_pth_offset - self.load_adress;
            let offset: u16 = self.song[offset_low_file + i] as u16
                | (self.song[offset_high_file + i] as u16) << 8;
            tracks_ptr.push(offset);
        }

        let mut tracks: Vec<Vec<u8>> = vec![];
        for i in 0..self.patt_qty {
            let file_offset = 126 + tracks_ptr[i] as usize - self.load_adress;
            let mut track: Vec<u8> = vec![];
            let mut j = 0;
            loop {
                track.push(self.song[file_offset + j]);
                if self.song[file_offset + j] == 0xff {
                    break;
                }
                j += 1;
            }
            tracks.push(track);
        }

        // ********* INSTRUMENTS

        let mut instruments: Vec<InstrRobSid> = vec![];
        for i in 0..self.instr_qty {
            let file_offset = 126 + self.instr_offset - self.load_adress;
            let start = file_offset + i * 8;

            let mut voice = SidVoice::default();
            voice.pw = self.song[start] as u16 | (self.song[start + 1] as u16) << 8;
            voice.update_from_ctrl_register(self.song[start + 2]);
            voice.ad = self.song[start + 3];
            voice.sr = self.song[start + 4];

            let mut isid = InstrSid::default();
            isid.voice[0] = voice;

            let mut re = RobEffects::default();
            if self.version == 10 {
                re.vibrato_depth = self.song[start + 5] << 2; // original seems 0..3
                re.vibrato_div = 7;
            } else {
                re.vibrato_depth = (self.song[start + 5] & 0b0_1111_000) >> 3;
                re.vibrato_div = self.song[start + 5] & 0b0000_111;
            }
            if re.vibrato_depth != 0 {
                re.vibrato = true;
            }

            let mask_lo: u8 = if let 15 = self.version {
                0b0000_1111
            } else {
                0b000_11111
            };
            let mask_hi: u8 = if let 15 = self.version {
                0b1111_0000
            } else {
                0b111_00000
            };

            re.pw_delay = (self.song[start + 6] & mask_lo) as u16;
            re.pw_speed = (self.song[start + 6] & mask_hi) as i8;

            //FIXME
            let commando_change_pw_effect = (self.song[start + 7] & 0b0000_1000) != 0;
            let fx_use = (self.song[start + 7] & 0b00_111_000) >> 3;
            if self.song[start + 7] & 0b0000_0001 != 0 {
                re.drum = true;
            }
            if self.song[start + 7] & 0b0000_0010 != 0 {
                re.skydive = true;
            }
            if self.song[start + 7] & 0b0000_0100 != 0 {
                re.arpeggio = true;
            }

            re.skydive_config_if = self.skydive_v1_when as u8;
            re.skydive_config_add = self.skydive_v1_add as u8;

            let mut irsid = InstrRobSid::default();
            irsid.sid = isid;
            irsid.fx[0] = re;

            instruments.push(irsid);
        }

        // ********* SOUNDFX
        let mut sfxs: Vec<SoundFx> = vec![];
        for i in 0..self.fx_v1_qty {
            let file_offset = 126 + self.fx_v1_offset - self.load_adress;
            let start = file_offset + i * 16;

            // play current music freq at end of incdec_counter counter
            let incdec_start_at_end = (self.song[start] & 0b1000_0000) == 0;
            let incdec_counter = if (self.song[start] & 0b0011_0000) == 0b0010_0000 {
                self.song[start] as i8 & 0b0000_1111
            } else {
                -(self.song[start] as i8 & 0b0000_1111)
            };
            let note_start = self.song[start + 1]; // as u16|(self.song[start+2] as u16)<<8;
            let note_delta = self.song[start + 8] & 0b0011_1111;
            let note_end = self.song[start + 15] & 0b00_111111;
            let flipflop_voice1_ctrl =
                ((self.song[start + 8] as u16 | (self.song[start + 9] as u16) << 8) & 0b0100_0000)
                    != 0;
            let voice0_ctrl = (self.song[start + 15] & 0b1000_0000) != 0;
            let voice1_ctrl = (self.song[start + 15] & 0b0100_0000) != 0;

            let mut voice0 = SidVoice::default();
            voice0.pw = self.song[start + 3] as u16 | (self.song[start + 4] as u16) << 8;
            voice0.update_from_ctrl_register(self.song[start + 5]);
            voice0.ad = self.song[start + 6];
            voice0.sr = self.song[start + 7];

            let mut voice1 = SidVoice::default();
            voice1.pw = self.song[start + 10] as u16 | (self.song[start + 11] as u16) << 8;
            voice0.update_from_ctrl_register(self.song[start + 12]);
            voice1.ad = self.song[start + 13];
            voice1.sr = self.song[start + 14];

            let sfx = SoundFx {
                incdec_start_at_end,
                incdec_counter,
                note_start,
                note_delta,
                note_end,
                flipflop_voice1_ctrl,
                voice0_ctrl,
                voice1_ctrl,
                voice0,
                voice1,
            };
            sfxs.push(sfx);
        }

        SidModule {
            sid: self.clone(),
            pattern_helper: PatternHelper::new(self.version, songs, channels, tracks),
            instruments,
            soundfx: sfxs,
        }
    }
}

impl OneSid {
    pub fn get_sid_commando() -> Self {
        OneSid {
            song: include_bytes!("songs/commando.sid"),
            name: "Commando",
            author: "Rob Hubbard",
            copyright: "1985 Elite",
            version: 10,
            load_adress: 0x5000,
            song_track_qty: 3,
            song_list_offset: 0x56FF,
            song_list_qty: 3,
            patt_ptl_offset: 0x5711,
            patt_pth_offset: 0x573E,
            patt_qty: 45,
            instr_offset: 0x5591,
            instr_qty: 13,
            fx_v1_offset: 0x55F9,
            fx_v1_qty: 16,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 2, // [2, 3, 2]
            skydive_v1_when: 2,
            skydive_v1_add: 512,
        }
    }

    pub fn get_sid_crazy_comets() -> Self {
        OneSid {
            song: include_bytes!("songs/crazy_comets.sid"),
            name: "Crazy Comets",
            author: "Rob Hubbard",
            copyright: "1985 Martech",
            version: 10,
            load_adress: 0x5000,
            song_track_qty: 3,
            song_list_offset: 0x5732,
            song_list_qty: 2,
            patt_ptl_offset: 0x573E,
            patt_pth_offset: 0x5773,
            patt_qty: 53,
            instr_offset: 0x5574,
            instr_qty: 23,
            fx_v1_offset: 0x562C,
            fx_v1_qty: 1,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 2,
            skydive_v1_when: 16,
            skydive_v1_add: -256,
        }
    }

    pub fn get_sid_last_v8() -> Self {
        OneSid {
            song: include_bytes!("songs/last_v8.sid"),
            name: "The Last V8",
            author: "Rob Hubbard",
            copyright: "1985 MAD/Mastertronic",
            version: 10,
            load_adress: 0x8010,
            song_track_qty: 3,
            song_list_offset: 0x8797,
            song_list_qty: 3,
            patt_ptl_offset: 0x87A9,
            patt_pth_offset: 0x87C6,
            patt_qty: 28,
            instr_offset: 0x85A1,
            instr_qty: 19,
            fx_v1_offset: 0x8699,
            fx_v1_qty: 12,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: -256,
        }
    }

    pub fn get_sid_monty_on_the_run() -> Self {
        OneSid {
            song: include_bytes!("songs/monty_on_the_run.sid"),
            name: "Monty on the Run",
            author: "Rob Hubbard",
            copyright: "1985 Gremlin Graphics",
            version: 10,
            load_adress: 0x8000,
            song_track_qty: 3,
            song_list_offset: 0x856C,
            song_list_qty: 3,
            patt_ptl_offset: 0x857E,
            patt_pth_offset: 0x85CB,
            patt_qty: 77,
            instr_offset: 0x93B4,
            instr_qty: 20,
            fx_v1_offset: 0x9454,
            fx_v1_qty: 16,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: -256,
        }
    }

    pub fn get_sid_thing_on_a_spring() -> Self {
        OneSid {
            song: include_bytes!("songs/thing_on_a_spring.sid"),
            name: "Thing on a Spring",
            author: "Rob Hubbard",
            copyright: "1985 Gremlin Graphics",
            version: 10,
            load_adress: 0xC000,
            song_track_qty: 3,
            song_list_offset: 0xC509,
            song_list_qty: 1,
            patt_ptl_offset: 0xC50F,
            patt_pth_offset: 0xC533,
            patt_qty: 36,
            instr_offset: 0xCD2A,
            instr_qty: 45,
            fx_v1_offset: 0xCE92,
            fx_v1_qty: 1,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_zoid() -> Self {
        OneSid {
            song: include_bytes!("songs/zoids.sid"),
            name: "Zoids",
            author: "Rob Hubbard",
            copyright: "1986 Martech",
            version: 10,
            load_adress: 0x1000,
            song_track_qty: 3,
            song_list_offset: 0x14FC,
            song_list_qty: 3,
            patt_ptl_offset: 0x150E,
            patt_pth_offset: 0x152D,
            patt_qty: 31,
            instr_offset: 0x147E,
            instr_qty: 15,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    //------------------------------------------

    pub fn get_sid_ace_2() -> Self {
        OneSid {
            song: include_bytes!("songs/ace_2.sid"),
            name: "ACE II",
            author: "Rob Hubbard",
            copyright: "1987 Arcade",
            version: 20,
            load_adress: 0xE000,
            song_track_qty: 3,
            song_list_offset: 0xE67C,
            song_list_qty: 1,
            patt_ptl_offset: 0xE682,
            patt_pth_offset: 0xE6A8,
            patt_qty: 38,
            instr_offset: 0xE5CB,
            instr_qty: 10,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_delta() -> Self {
        OneSid {
            song: include_bytes!("songs/delta.sid"),
            name: "Delta",
            author: "Rob Hubbard",
            copyright: "1987 Thalamus",
            version: 30, // Compression _and_ pattern loop in channels
            load_adress: 0xBC00,
            song_track_qty: 3,
            song_list_offset: 0xC4F4,
            song_list_qty: 13,
            patt_ptl_offset: 0xC542,
            patt_pth_offset: 0xC5AF,
            patt_qty: 109,
            instr_offset: 0xC38E,
            instr_qty: 22,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0xC43E,
            fx_v2_qty: 22,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_human_race() -> Self {
        OneSid {
            song: include_bytes!("songs/human_race.sid"),
            name: "The Human Race",
            author: "Rob Hubbard",
            copyright: "1985 Mastertronic",
            version: 20,
            load_adress: 0x0980,
            song_track_qty: 2,
            song_list_offset: 0x0E9F,
            song_list_qty: 3,
            patt_ptl_offset: 0x0EB3,
            patt_pth_offset: 0x0F02,
            patt_qty: 58,
            instr_offset: 0x0DE3,
            instr_qty: 23,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 3,
            skydive_v1_when: 0,
            skydive_v1_add: -256,
        }
    }

    pub fn get_sid_international_karate() -> Self {
        OneSid {
            song: include_bytes!("songs/international_karate.sid"),
            name: "International Karate",
            author: "Rob Hubbard",
            copyright: "1986 System 3",
            version: 20,
            load_adress: 0xAE00,
            song_track_qty: 3,
            song_list_offset: 0xB3B0,
            song_list_qty: 1,
            patt_ptl_offset: 0xB3B6,
            patt_pth_offset: 0xB3EB,
            patt_qty: 53,
            instr_offset: 0xB308,
            instr_qty: 20,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 2,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_lightforce() -> Self {
        OneSid {
            song: include_bytes!("songs/lightforce.sid"),
            name: "Lightforce",
            author: "Rob Hubbard",
            copyright: "1986 Faster Than Light (FTL)",
            version: 20,
            load_adress: 0xF000,
            song_track_qty: 3,
            song_list_offset: 0xF778,
            song_list_qty: 1,
            patt_ptl_offset: 0xF77E,
            patt_pth_offset: 0xF79D,
            patt_qty: 31,
            instr_offset: 0xF618,
            instr_qty: 22,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0xF6C8,
            fx_v2_qty: 22,
            resetspd: 2,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_sanxion_song_1() -> Self {
        OneSid {
            song: include_bytes!("songs/sanxion.sid"),
            name: "Sanxion Song 1",
            author: "Rob Hubbard",
            copyright: "1986 Thalamus",
            version: 20,
            load_adress: 0xB000,
            song_track_qty: 3,
            song_list_offset: 0xB73C,
            song_list_qty: 1,
            patt_ptl_offset: 0xB742,
            patt_pth_offset: 0xB75D,
            patt_qty: 27,
            instr_offset: 0xB56C,
            instr_qty: 29,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0xB654,
            fx_v2_qty: 29,
            resetspd: 2,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_sanxion_song_2() -> Self {
        OneSid {
            song: include_bytes!("songs/sanxion.sid"),
            name: "Sanxion Song 2",
            author: "Rob Hubbard",
            copyright: "1986 Thalamus",
            version: 20,
            load_adress: 0xB000,
            song_track_qty: 3,
            song_list_offset: 0xC5F5,
            song_list_qty: 1,
            patt_ptl_offset: 0xC5FB,
            patt_pth_offset: 0xC633,
            patt_qty: 56,
            instr_offset: 0xC4B5,
            instr_qty: 20,
            fx_v1_offset: 0,
            fx_v1_qty: 0,
            fx_v2_offset: 0xC5F5,
            fx_v2_qty: 20,
            resetspd: 2,
            skydive_v1_when: 0,
            skydive_v1_add: 0,
        }
    }

    pub fn get_sid_spellbound() -> Self {
        OneSid {
            song: include_bytes!("songs/spellbound.sid"),
            name: "Spellbound",
            author: "Rob Hubbard",
            copyright: "1986 MAD/Mastertronic",
            version: 15, // XXX WARN: 0xE0E5 second byte & 0b1000_0000 is instrnr: not a vibrato! no vibrato. Using version 15 -- check soundfx too
            load_adress: 0xE000,
            song_track_qty: 3,
            song_list_offset: 0xE6B6,
            song_list_qty: 3,
            patt_ptl_offset: 0xE6C8,
            patt_pth_offset: 0xE6F2,
            patt_qty: 42,
            instr_offset: 0xE548,
            instr_qty: 25,
            fx_v1_offset: 0xE610,
            fx_v1_qty: 16,
            fx_v2_offset: 0,
            fx_v2_qty: 0,
            resetspd: 1,
            skydive_v1_when: 0,
            skydive_v1_add: 256,
        }
    }
}
