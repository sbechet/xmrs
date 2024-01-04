#![forbid(unsafe_code)]

use bincode::ErrorKind;
use xmrs::xm::xiinstrument::XiInstrument;

const XI: &[u8] = include_bytes!("instr.xi");

fn main() -> Result<(), Box<ErrorKind>> {
    let xmi = XiInstrument::load(XI)?;
    println!("Load XMI: {:#x?}", xmi);
    let instr = xmi.to_instrument();
    println!("Convert to instrument: {:#x?}", instr);

    Ok(())
}
