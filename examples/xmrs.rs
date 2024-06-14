#![forbid(unsafe_code)]

use bincode::ErrorKind;
use std::fs::File;
use std::io::prelude::*;
use xmrs::module::Module;
use xmrs::xm::xmmodule::XmModule;

const XM: &[u8] = include_bytes!("note.xm");

fn main() -> Result<(), Box<ErrorKind>> {
    let xmmodule: XmModule = XmModule::load(XM)?;
    println!("Load XM: {:#x?}", xmmodule);
    let module: Module = xmmodule.to_module();
    println!("Convert to module: {:#x?}", module);

    let mut xmmodule2: XmModule = XmModule::from_module(&module);
    println!("Convert back to XM: {:#x?}", xmmodule2);

    let xmodule2_se = xmmodule2.save()?;
    let mut file = File::create("output_debug.xm")?;
    file.write_all(&xmodule2_se)?;
    println!("Save XM file to `output_debug.xm`");

    let xmmodule3: XmModule = XmModule::load(&xmodule2_se)?;
    println!("Load Again: {:#x?}", xmmodule3);

    Ok(())
}
