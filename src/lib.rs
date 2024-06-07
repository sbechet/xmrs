#![forbid(unsafe_code)]
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

#![no_std]
extern crate alloc;

/// Envelope with Steroid
pub mod envelope;
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
/// Vibrato with Steroid
pub mod instr_vibrato;
/// Instrument with Steroid
pub mod instrument;
/// SoundTracker Module with Steroid
pub mod module;
/// A typical Note
pub mod note;
/// A typical pattern slot
pub mod patternslot;
/// Sample with Steroid
pub mod sample;

/// Load and Save Historical XM files
#[cfg(feature = "loaders")]
pub mod xm;

/// Load only Historical MOD files
#[cfg(feature = "loaders")]
pub mod amiga;

pub mod f32helper;

/// The Xmrs Prelude
pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(42, 42);
    }
}
