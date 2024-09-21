use serde::{Deserialize, Serialize};

#[cfg(feature = "micromath")]
use micromath::F32Ext;
#[cfg(feature = "libm")]
use num_traits::float::Float;



/// Historical Frequencies to load old data. Default is Linear.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum FrequencyType {
    AmigaFrequencies,
    LinearFrequencies,
}

impl Default for FrequencyType {
    fn default() -> Self {
        Self::LinearFrequencies
    }
}

#[derive(Clone)]
pub struct PeriodHelper {
    pub freq_type: FrequencyType,
    historical: bool,
}

impl Default for PeriodHelper {
    fn default() -> Self {
        Self {
            freq_type: FrequencyType::LinearFrequencies,
            historical: false,
        }
    }
}

impl PeriodHelper {
    pub const C4_FREQ:f32 = 8363.0; // historical amiga module sample frequency (Paula chipset related)

    pub fn new(freq_type: FrequencyType, historical: bool) -> Self {
        Self {
            freq_type,
            historical,
        }
    }

    // ==== Linear

    /// return period
    #[inline(always)]
    fn linear_note_to_period(note: f32) -> f32 {
        // 10.0: number of octaves
        // 12.0: halftones
        // 16.0: number of finetune steps
        //  4.0: finetune resolution
        10.0 * 12.0 * 16.0 * 4.0 - note * 16.0 * 4.0
    }

    /// return note
    #[inline(always)]
    fn linear_period_to_note(period: f32) -> f32 {
        (10.0 * 12.0 * 16.0 * 4.0 - period) / (16.0 * 4.0)
    }

    /// return frequency
    #[inline(always)]
    fn linear_period_to_frequency(period: f32) -> f32 {
        // 8363.0 is historical amiga module sample frequency (Paula chipset related)
        //  6: octave center
        // 12: halftones
        // 64: period resolution (16.0 * 4.0)
        //     16.0: number of finetune steps
        //      4.0: finetune step resolution
        Self::C4_FREQ * (2.0f32).powf((6.0 * 12.0 * 16.0 * 4.0 - period) / (12.0 * 16.0 * 4.0))
    }

    /// return period
    #[inline(always)]
    fn linear_frequency_to_period(freq: f32) -> f32 {
        (6.0 * 12.0 * 16.0 * 4.0) - (12.0 * 16.0 * 4.0) * (freq / Self::C4_FREQ).log2()
    }


    // ==== Amiga

    /// return period
    #[inline(always)]
    fn amiga_note_to_period(note: f32) -> f32 {
        /* found using scipy.optimize.curve_fit */
        6848.0 * (-0.0578 * note).exp() + 0.2782
    }

    /// return note
    #[inline(always)]
    fn amiga_period_to_note(period: f32) -> f32 {
        -f32::ln((period - 0.2782) / 6848.0) / 0.0578
    }

    /// return frequency
    #[inline(always)]
    fn amiga_period_to_frequency(period: f32) -> f32 {
        if period == 0.0 {
            0.0
        } else {
            // 7159090.5 / (period * 2.0) // NTSC
            7093789.2 / (period * 2.0) // PAL
        }
    }

    /// return period
    #[inline(always)]
    fn amiga_frequency_to_period(freq: f32) -> f32 {
        if freq == 0.0 {
            0.0
        } else {
            // 7159090.5 / (freq * 2.0) // NTSC
            7093789.2 / (freq * 2.0) // PAL
        }
    }

    // ==== Generic (TODO: use a trait any day?)

    pub fn note_to_period(&self, note: f32) -> f32 {
        match self.freq_type {
            FrequencyType::LinearFrequencies => Self::linear_note_to_period(note),
            FrequencyType::AmigaFrequencies => Self::amiga_note_to_period(note),
        }
    }

    pub fn period_to_note(&self, period: f32) -> f32 {
        match self.freq_type {
            FrequencyType::LinearFrequencies => Self::linear_period_to_note(period),
            FrequencyType::AmigaFrequencies => Self::amiga_period_to_note(period),
        }
        .max(0.0) // Remove < 0.0 and NaN numbers
    }

    pub fn period_to_frequency(&self, period: f32) -> f32 {
        match self.freq_type {
            FrequencyType::LinearFrequencies => Self::linear_period_to_frequency(period),
            FrequencyType::AmigaFrequencies => Self::amiga_period_to_frequency(period),
        }
    }

    pub fn frequency_to_period(&self, freq: f32) -> f32 {
        match self.freq_type {
            FrequencyType::LinearFrequencies => Self::linear_frequency_to_period(freq),
            FrequencyType::AmigaFrequencies => Self::amiga_frequency_to_period(freq),
        }
    }

    /// returns C-4 frequency from relative note and finetune
    pub fn relative_note_to_c4freq(&self, relative_note: f32, finetune: f32) -> Option<f32> {
        const NOTE_C4: f32 = 4.0 * 12.0;
        const NOTE_B9: f32 = 10.0 * 12.0 - 1.0;

        let note = NOTE_C4 + relative_note;
        if note < 0.0 || note > NOTE_B9 {
            return None;
        }
        let c4_period = self.note_to_period(note + finetune);
        Some(self.period_to_frequency(c4_period))
    }

    /// return relative note including finetune
    pub fn c4freq_to_relative_note(&self, freq: f32) -> f32 {
        const NOTE_C4: f32 = 4.0 * 12.0;
        let period = self.frequency_to_period(freq);
        let note = self.period_to_note(period);
        note - NOTE_C4
    }

    //-----------------------------------------------------

    // new adjust period to arpeggio and finetune delta
    pub fn adjust_period(&self, period: f32, arp_note: f32, finetune: f32, semitone: bool) -> f32 {
        let note_orig: f32 = self.period_to_note(period);

        let note = if semitone {
            note_orig.round()
        } else {
            note_orig
        };

        if self.historical && arp_note != 0.0 {
            // From C-0 (0) to B-7 (95) only
            let mut note = note;
            if note.ceil() >= 95.0 {
                note = 95.0;
            }
            self.note_to_period(note + arp_note + finetune)
        } else {
            self.note_to_period(note + arp_note + finetune)
        }
    }
}
