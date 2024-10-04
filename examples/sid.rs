#![forbid(unsafe_code)]

use bincode::error::EncodeError;
use std::fs::File;
use std::io::prelude::*;
use xmrs::module::Module;
use xmrs::sid::sid_module::SidModule;
use xmrs::xm::xmmodule::XmModule;

fn save_xm(sid: &SidModule) -> Result<(), EncodeError> {
    let modules: Vec<Module> = sid.to_module();
    for module in &modules {
        let mut xmmodule: XmModule = XmModule::from_module(&module);
        let xmodule_se = xmmodule.save()?;
        let filename = format!("{}.xm", module.name);
        println!("Saving {}`", filename);
        let mut file = File::create(filename).unwrap();
        file.write_all(&xmodule_se).unwrap();
    }

    Ok(())
}

fn main() -> Result<(), EncodeError> {
    println!("--===~ XmRs SID Module Info Example ~===--");
    println!("(c) 2024 Sébastien Béchet\n");

    // println!("{:?}", sid);
    //TODO: SOUNDFX

    println!("Warning: XM is limited to 256 rows");

    save_xm(&SidModule::get_sid_commando())?;
    save_xm(&SidModule::get_sid_crazy_comets())?;
    save_xm(&SidModule::get_sid_monty_on_the_run())?;
    save_xm(&SidModule::get_sid_last_v8())?;
    save_xm(&SidModule::get_sid_thing_on_a_spring())?;
    save_xm(&SidModule::get_sid_zoid())?;
    // WIP
    // save_xm(&SidModule::get_sid_ace_2())?;
    // save_xm(&SidModule::get_sid_delta())?;
    // save_xm(&SidModule::get_sid_human_race())?;
    // save_xm(&SidModule::get_sid_international_karate())?;
    // save_xm(&SidModule::get_sid_lightforce())?;
    // save_xm(&SidModule::get_sid_sanxion_song_1())?;
    // save_xm(&SidModule::get_sid_sanxion_song_2())?;
    // save_xm(&SidModule::get_sid_spellbound())?;

    Ok(())
}
