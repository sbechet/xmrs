//!
//! XMrs is a Safe SoundTracker Library
//! 
//! ```
//! module+--->instrument+--->instr_defaut+--->sample
//!       |              |                +--->envelope
//!       |              |                +--->vibrato
//!       |              +--->instr_ekn
//!       |              +--->instr_midi
//!       |              +--->instr_sid
//!       |              +-+->instr_robrs
//!       |                +--->instr_sid
//!       +--->Pattern--->Row--->patternslot
//! ```
//! 
//! You can load (and save) historical XM files using `xm` (see `README.md`)
//! 
//! You can load (and save) your work using `load()` and `save()` serde fn
//! 


/// Envelope with Steroid
pub mod envelope;
/// Instrument with Steroid
pub mod instrument;
/// SoundTracker Module with Steroid
pub mod module;
/// A typical pattern slot
pub mod patternslot;
/// Sample with Steroid
pub mod sample;
/// Vibrato with Steroid
pub mod vibrato;
/// Historical XM Instrument
pub mod instr_default;
/// Euclidian Rythm Instrument
pub mod instr_ekn;
/// Midi Instrument
pub mod instr_midi;
/// Rob Hubbard Instrument
pub mod instr_robsid;
/// MOS6581 SID Instrument
pub mod instr_sid;

/// Load and Save Historical XM files
pub mod xm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, 42);
    }
}
