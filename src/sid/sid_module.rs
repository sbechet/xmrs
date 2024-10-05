use crate::{instr_robsid::RobEffects, instr_sid::SidVoice, prelude::*};
use alloc::format;
use alloc::string::String;
use alloc::{vec, vec::Vec};

use crate::prelude::*;

use super::instr_helper::InstrHelper;
use super::one_sid::OneSid;
use super::pattern_helper::PatternHelper;
use super::sound_fx::SoundFx;

#[derive(Debug)]
pub struct SidModule {
    pub sid: OneSid,
    pub pattern_helper: PatternHelper,
    pub instruments: Vec<InstrRobSid>,
    pub soundfx: Vec<SoundFx>,
}

impl SidModule {
    pub fn to_modules(&self, original_instruments: bool) -> Vec<Module> {
        let mut modules: Vec<Module> = vec![];

        for song_number in 0..self.pattern_helper.songs.len() {
            let mut module = Module::default();
            module.name = format!("{} {}", self.sid.name, song_number);
            module.comment = format!(
                "{} - {} (song #{})",
                self.sid.copyright, self.sid.author, song_number
            );
            module.default_tempo = (1 + self.sid.resetspd) as u16;

            let patterns = self.pattern_helper.get_patterns(song_number);
            let (patterns, pattern_order) = PatternHelper::cleanup_patterns(&patterns);

            module.pattern = patterns;
            module.pattern_order = pattern_order;

            let idst = InstrHelper::irss_to_instruments(&self.instruments, original_instruments);
            module.instrument = idst;

            modules.push(module);
        }

        return modules;
    }
}

impl SidModule {
    pub fn get_sid_commando() -> Self {
        let sid = OneSid::get_sid_commando();
        return sid.to_sidmodule();
    }

    pub fn get_sid_crazy_comets() -> Self {
        let sid = OneSid::get_sid_crazy_comets();
        return sid.to_sidmodule();
    }

    pub fn get_sid_last_v8() -> Self {
        let sid = OneSid::get_sid_last_v8();
        return sid.to_sidmodule();
    }

    pub fn get_sid_monty_on_the_run() -> Self {
        let sid = OneSid::get_sid_monty_on_the_run();
        return sid.to_sidmodule();
    }

    pub fn get_sid_thing_on_a_spring() -> Self {
        let sid = OneSid::get_sid_thing_on_a_spring();
        return sid.to_sidmodule();
    }

    pub fn get_sid_zoid() -> Self {
        let sid = OneSid::get_sid_zoid();
        return sid.to_sidmodule();
    }

    //--------------------------
    // WIP

    pub fn get_sid_ace_2() -> Self {
        let sid = OneSid::get_sid_ace_2();
        return sid.to_sidmodule();
    }

    pub fn get_sid_delta() -> Self {
        let sid = OneSid::get_sid_delta();
        return sid.to_sidmodule();
    }

    pub fn get_sid_human_race() -> Self {
        let sid = OneSid::get_sid_human_race();
        return sid.to_sidmodule();
    }

    pub fn get_sid_international_karate() -> Self {
        let sid = OneSid::get_sid_international_karate();
        return sid.to_sidmodule();
    }

    pub fn get_sid_lightforce() -> Self {
        let sid = OneSid::get_sid_lightforce();
        return sid.to_sidmodule();
    }

    pub fn get_sid_sanxion_song_1() -> Self {
        let sid = OneSid::get_sid_sanxion_song_1();
        return sid.to_sidmodule();
    }

    pub fn get_sid_sanxion_song_2() -> Self {
        let sid = OneSid::get_sid_sanxion_song_2();
        return sid.to_sidmodule();
    }

    pub fn get_sid_spellbound() -> Self {
        let sid = OneSid::get_sid_sanxion_song_2();
        return sid.to_sidmodule();
    }
}
