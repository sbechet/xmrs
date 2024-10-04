#![forbid(unsafe_code)]

use bincode::error::DecodeError;
use xmrs::xm::xi_instrument::XiInstrument;

const XI: &[u8] = include_bytes!("instr.xi");

fn main() -> Result<(), DecodeError> {
    let xmi = XiInstrument::load(XI)?;
    println!("Load XMI: {:#x?}", xmi);
    let instr = xmi.to_instrument();
    println!("Convert to instrument: {:#x?}", instr);

    Ok(())
}
