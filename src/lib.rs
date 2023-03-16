pub mod module;
pub mod patternslot;
pub mod instrument;
pub mod envelope;
pub mod vibrato;
pub mod sample;

pub mod instr_ekn;
pub mod instr_midi;
pub mod instr_sid;
pub mod instr_robsid;

// load and save xm
pub mod xm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, 42);
    }
}
