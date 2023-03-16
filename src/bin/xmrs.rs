use std::fs::File;
use std::io::prelude::*;
use bincode::ErrorKind;
use xmrs::xm::xmmodule::XmModule;
use xmrs::module::Module;


const XM: &[u8] = include_bytes!("note.xm");

fn main() -> Result<(), Box<ErrorKind>> {
    let xmmodule: XmModule = XmModule::load(&XM)?;
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

    // Not to create a new file format but because it's sooo simple to use bincode.
    let bcok = module.save()?;
    let mut file = File::create("output_debug.xmrs")?;
    file.write_all(&bcok)?;
    println!("Create bincode serialized module");

    let bcoks = bcok.as_slice();
    let bcread_xmrs = Module::load(bcoks)?;

    println!("reread from bincode: {:x?}", bcread_xmrs);


    Ok(())
}


