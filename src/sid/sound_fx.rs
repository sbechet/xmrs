use crate::instr_sid::SidVoice;

#[derive(Debug)]
pub struct SoundFx {
    pub incdec_start_at_end: bool,
    pub incdec_counter: i8,
    pub note_start: u8,
    pub note_delta: u8,
    pub note_end: u8,
    pub flipflop_voice1_ctrl: bool,
    pub voice0_ctrl: bool,
    pub voice1_ctrl: bool,
    pub voice0: SidVoice,
    pub voice1: SidVoice,
}
