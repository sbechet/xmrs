use alloc::vec::Vec;

/*
 *   Code stolen from `ft2_load_s3m.c` and converted to rust using an LLM with a small modification
 *   AI GENERATED
 *   BSD-3-Clause license
 *
 */
use crate::prelude::*;

#[derive(Debug)]
pub struct S3mEffect {
    alastnfo: [u8; 32],
    alastefx: [u8; 32],
    alastvibnfo: [u8; 32],
    s3m_last_g_instrument: [u8; 32],
}

impl S3mEffect {
    fn new() -> Self {
        Self {
            alastnfo: [0; 32],
            alastefx: [0; 32],
            alastvibnfo: [0; 32],
            s3m_last_g_instrument: [0; 32],
        }
    }

    pub fn update_pattern(pattern: &mut Vec<Vec<PatternSlot>>) {
        let mut ufx = Self::new();
        for line in pattern {
            for (ii, n) in line.iter_mut().enumerate() {
                ufx.efx_correction(ii, n);
            }
        }
    }

    /* ii is the channel index
     */
    fn efx_correction(&mut self, ii: usize, n: &mut PatternSlot) {
        if n.effect_parameter > 0 {
            self.alastnfo[ii] = n.effect_parameter;
            if n.effect_type == 8 || n.effect_type == 21 {
                self.alastvibnfo[ii] = n.effect_parameter; // H/U
            }
        }

        // in ST3, a lot of effects directly share the same memory!
        if n.effect_parameter == 0 && n.effect_type != 7 {
            // G
            let efx = n.effect_type;
            if efx == 8 || efx == 21 {
                // H/U
                n.effect_parameter = self.alastvibnfo[ii];
            } else if (4..=12).contains(&efx) || (17..=19).contains(&efx) {
                // D/E/F/I/J/K/L/Q/R/S
                n.effect_parameter = self.alastnfo[ii];
            }

            /* If effect data is zero and effect type was the same as last one, clear out
            	** data if it's not J or S (those have no memory in the equivalent XM effects).
            	** Also goes for extra fine pitch slides and fine volume slides,
             ** since they get converted to other effects.
             */
            if efx == self.alastefx[ii] && efx != 10 && efx != 19 {
                // J/S
                let nfo = n.effect_parameter;
                let extra_fine_pitch_slides = (efx == 5 || efx == 6) && ((nfo & 0xF0) == 0xE0);
                let fine_vol_slides = (efx == 4 || efx == 11)
                    && ((nfo > 0xF0) || (((nfo & 0xF) == 0xF) && ((nfo & 0xF0) > 0)));

                if !extra_fine_pitch_slides && !fine_vol_slides {
                    n.effect_parameter = 0;
                }
            }
        }

        if n.effect_type > 0 {
            self.alastefx[ii] = n.effect_type;
        }

        match n.effect_type {
            1 => {
                // A
                n.effect_type = 0xF;
                if n.effect_parameter == 0 {
                    n.effect_type = 0;
                    n.effect_parameter = 0;
                } else if n.effect_parameter > 0x1F {
                    n.effect_parameter = 0x1F;
                }
            }
            2 => n.effect_type = 0xB, // B
            3 => n.effect_type = 0xD, // C
            4 => {
                // D
                if n.effect_parameter > 0xF0 {
                    // fine slide up
                    n.effect_type = 0xE;
                    n.effect_parameter = 0xB0 | (n.effect_parameter & 0xF);
                } else if (n.effect_parameter & 0x0F) == 0x0F && (n.effect_parameter & 0xF0) > 0 {
                    // fine slide down
                    n.effect_type = 0xE;
                    n.effect_parameter = 0xA0 | (n.effect_parameter >> 4);
                } else {
                    n.effect_type = 0xA;
                    if n.effect_parameter & 0x0F != 0 {
                        // on D/K, last nybble has first priority in ST3
                        n.effect_parameter &= 0x0F;
                    }
                }
            }
            5 | 6 => {
                // E, F
                if (n.effect_parameter & 0xF0) >= 0xE0 {
                    // convert to fine slide
                    let tmp = if (n.effect_parameter & 0xF0) == 0xE0 {
                        0x21
                    } else {
                        0x0E
                    };

                    n.effect_parameter &= 0x0F;

                    if n.effect_type == 5 {
                        n.effect_parameter |= 0x20;
                    } else {
                        n.effect_parameter |= 0x10;
                    }

                    n.effect_type = tmp;

                    if n.effect_type == 0x21 && n.effect_parameter == 0 {
                        n.effect_type = 0;
                    }
                } else {
                    n.effect_type = 7 - n.effect_type; // convert to normal 1xx/2xx slide
                }
            }
            7 => {
                // G
                n.effect_type = 0x03;

                // fix illegal slides (to new instruments)
                if n.instrument != 0 && n.instrument != self.s3m_last_g_instrument[ii] {
                    n.instrument = self.s3m_last_g_instrument[ii];
                }
            }
            11 => {
                // K
                if n.effect_parameter > 0xF0 {
                    // fine slide up
                    n.effect_type = 0xE;
                    n.effect_parameter = 0xB0 | (n.effect_parameter & 0xF);

                    // if volume column is unoccupied, set to vibrato
                    if n.volume == 0 {
                        n.volume = 0xB0;
                    }
                } else if (n.effect_parameter & 0x0F) == 0x0F && (n.effect_parameter & 0xF0) > 0 {
                    // fine slide down
                    n.effect_type = 0xE;
                    n.effect_parameter = 0xA0 | (n.effect_parameter >> 4);

                    // if volume column is unoccupied, set to vibrato
                    if n.volume == 0 {
                        n.volume = 0xB0;
                    }
                } else {
                    n.effect_type = 0x6;
                    if n.effect_parameter & 0x0F != 0 {
                        // on D/K, last nybble has first priority in ST3
                        n.effect_parameter &= 0x0F;
                    }
                }
            }
            8 => n.effect_type = 0x04,  // H
            9 => n.effect_type = 0x1D,  // I
            10 => n.effect_type = 0x00, // J
            12 => n.effect_type = 0x05, // L
            15 => n.effect_type = 0x09, // O
            17 => n.effect_type = 0x1B, // Q
            18 => n.effect_type = 0x07, // R
            19 => {
                // S
                n.effect_type = 0xE;
                let tmp = n.effect_parameter >> 4;
                n.effect_parameter &= 0x0F;

                match tmp {
                    0x1 => n.effect_parameter |= 0x30,
                    0x2 => n.effect_parameter |= 0x50,
                    0x3 => n.effect_parameter |= 0x40,
                    0x4 => n.effect_parameter |= 0x70,
                    // ignore S8x becuase it's not compatible with FT2 panning
                    0xB => n.effect_parameter |= 0x60,
                    0xC => {
                        // Note Cut
                        n.effect_parameter |= 0xC0;
                        if n.effect_parameter == 0xC0 {
                            // EC0 does nothing in ST3 but cuts voice in FT2, remove effect
                            n.effect_type = 0;
                            n.effect_parameter = 0;
                        }
                    }
                    0xD => {
                        // Note Delay
                        n.effect_parameter |= 0xD0;
                        if let Note::None = n.note {
                            // EDx without a note does nothing in ST3 but retrigs in FT2, remove effect
                            n.effect_type = 0;
                            n.effect_parameter = 0;
                        } else if n.effect_parameter == 0xD0 {
                            // ED0 prevents note/smp/vol from updating in ST3, remove everything
                            n.note = Note::None;
                            n.instrument = 0;
                            n.volume = 0;
                            n.effect_type = 0;
                            n.effect_parameter = 0;
                        }
                    }
                    0xE => n.effect_parameter |= 0xE0,
                    0xF => n.effect_parameter |= 0xF0,
                    _ => {
                        n.effect_type = 0;
                        n.effect_parameter = 0;
                    }
                }
            }
            20 => {
                // T
                n.effect_type = 0x0F;
                if n.effect_parameter < 0x21 {
                    // Txx with a value lower than 33 (0x21) does nothing in ST3, remove effect
                    n.effect_type = 0;
                    n.effect_parameter = 0;
                }
            }
            22 => {
                // V
                n.effect_type = 0x10;
                if n.effect_parameter > 0x40 {
                    // Vxx > 0x40 does nothing in ST3
                    n.effect_type = 0;
                    n.effect_parameter = 0;
                }
            }
            _ => {
                n.effect_type = 0;
                n.effect_parameter = 0;
            }
        }

        if n.instrument != 0 && n.effect_type != 0x3 {
            self.s3m_last_g_instrument[ii] = n.instrument;
        }
    }
}
