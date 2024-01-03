use clap::Parser;
use std::sync::Arc;

use xmrs::prelude::*;
use xmrs::xm::xmmodule::XmModule;

#[derive(Parser)]
struct Cli {
    /// Choose XM or XmRs File
    #[arg(
        short = 'f',
        long,
        default_value = "coretex_-_home.xm", // https://modarchive.org/index.php?request=view_by_moduleid&query=159594
        value_name = "filename"
    )]
    filename: Option<String>,

    /// Turn pattern informations on
    #[arg(short = 'p', long, default_value = "false")]
    patterns: bool,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match cli.filename {
        Some(filename) => {
            println!("--===~ XmRs Info Example ~===--");
            println!("(c) 2024 Sébastien Béchet\n");
            // let path = std::env::current_dir()?;
            // println!("The current directory is {}", path.display());
            println!("opening {}", filename);
            let contents = std::fs::read(filename.trim())?;
            match XmModule::load(&contents) {
                Ok(xm) => {
                    drop(contents); // cleanup memory
                    let module = Arc::new(xm.to_module());
                    drop(xm);
                    println!("Module '{}' loaded...", module.name);
                    println!("comment: {}", module.comment);
                    println!("pattern_order: {:?}", module.pattern_order);
                    for mi in &module.instrument {
                        println!("======== New instrument name: {}", mi.name);
                        match &mi.instr_type {
                            InstrumentType::Empty => { println!("empty");
                            },
                            InstrumentType::Default(i) => {
                                println!("sample_for_note: {:?}",i.sample_for_note);
                                println!("volume_envelope: {:?}",i.volume_envelope);
                                println!("panning_envelope: {:?}",i.panning_envelope);
                                println!("vibrato: {:?}",i.vibrato);
                                println!("volume_fadeout: {:?}",i.volume_fadeout);
                                for is in &i.sample {
                                    println!("  ========== New sample name: {}", is.name);
                                    println!("loop_start: {}", is.loop_start);
                                    println!("loop_length: {}", is.loop_length);
                                    println!("sample len: {}", is.len());
                                    println!("volume: {}", is.volume);
                                    println!("finetune: {}", is.finetune);
                                    println!("flags: {:?}", is.flags);
                                    println!("relative_note: {}", is.relative_note);
                                }
                                println!("midi: {:?}",i.midi);
                                println!("midi_mute: {}",i.midi_mute_computer);
                            }
                            _ => {}
                        }
                    }
                    if cli.patterns {
                        for p in &module.pattern {
                            println!("{:?}",p);
                        }

                    }



                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        _ => {}
    }
    Ok(())
}

