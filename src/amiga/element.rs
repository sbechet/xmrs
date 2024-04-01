use std::fmt::*;

pub struct Element {
    pub note: u8,
    pub instrument: u8,
    pub effect: u8,
    pub data: u8,
}

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let note = match Self::note_to_text(self.note) {
            Some(n) => n,
            None => "---",
        };
        write!(
            f,
            "[{} {:02X} {:1X}{:02X}]",
            note, self.instrument, self.effect, self.data
        )
    }
}

impl Element {
    fn period_to_note(period: u16) -> Option<u8> {
        match period {
            1712 => Some(1),
            1616 => Some(2),
            1524 => Some(3),
            1440 => Some(4),
            1356 => Some(5),
            1280 => Some(6),
            1208 => Some(7),
            1140 => Some(8),
            1076 => Some(9),
            1016 => Some(10),
            960 => Some(11),
            906 => Some(12),
            856 => Some(13),
            808 => Some(14),
            762 => Some(15),
            720 => Some(16),
            678 => Some(17),
            640 => Some(18),
            604 => Some(19),
            570 => Some(20),
            538 => Some(21),
            508 => Some(22),
            480 => Some(23),
            453 => Some(24),
            428 => Some(25),
            404 => Some(26),
            381 => Some(27),
            360 => Some(28),
            339 => Some(29),
            320 => Some(30),
            302 => Some(31),
            285 => Some(32),
            269 => Some(33),
            254 => Some(34),
            240 => Some(35),
            226 => Some(36),
            214 => Some(37),
            202 => Some(38),
            190 => Some(39),
            180 => Some(40),
            170 => Some(41),
            160 => Some(42),
            151 => Some(43),
            143 => Some(44),
            135 => Some(45),
            127 => Some(46),
            120 => Some(47),
            113 => Some(48),
            107 => Some(49),
            101 => Some(50),
            95 => Some(51),
            90 => Some(52),
            85 => Some(53),
            80 => Some(54),
            75 => Some(55),
            71 => Some(56),
            67 => Some(57),
            63 => Some(58),
            60 => Some(59),
            56 => Some(60),
            _ => None,
        }
    }

    fn note_to_text(note: u8) -> Option<&'static str> {
        match note {
            1 => Some("C-0"),
            2 => Some("C#0"),
            3 => Some("D-0"),
            4 => Some("D#0"),
            5 => Some("E-0"),
            6 => Some("F-0"),
            7 => Some("F#0"),
            8 => Some("G-0"),
            9 => Some("G#0"),
            10 => Some("A-0"),
            11 => Some("A#0"),
            12 => Some("B-0"),
            13 => Some("C-1"),
            14 => Some("C#1"),
            15 => Some("D-1"),
            16 => Some("D#1"),
            17 => Some("E-1"),
            18 => Some("F-1"),
            19 => Some("F#1"),
            20 => Some("G-1"),
            21 => Some("G#1"),
            22 => Some("A-1"),
            23 => Some("A#1"),
            24 => Some("B-1"),
            25 => Some("C-2"),
            26 => Some("C#2"),
            27 => Some("D-2"),
            28 => Some("D#2"),
            29 => Some("E-2"),
            30 => Some("F-2"),
            31 => Some("F#2"),
            32 => Some("G-2"),
            33 => Some("G#2"),
            34 => Some("A-2"),
            35 => Some("A#2"),
            36 => Some("B-2"),
            37 => Some("C-3"),
            38 => Some("C#3"),
            39 => Some("D-3"),
            40 => Some("D#3"),
            41 => Some("E-3"),
            42 => Some("F-3"),
            43 => Some("F#3"),
            44 => Some("G-3"),
            45 => Some("G#3"),
            46 => Some("A-3"),
            47 => Some("A#3"),
            48 => Some("B-3"),
            49 => Some("C-4"),
            50 => Some("C#4"),
            51 => Some("D-4"),
            52 => Some("D#4"),
            53 => Some("E-4"),
            54 => Some("F-4"),
            55 => Some("F#4"),
            56 => Some("G-4"),
            57 => Some("G#4"),
            58 => Some("A-4"),
            59 => Some("A#4"),
            60 => Some("B-4"),
            _ => None,
        }
    }

    /*
        0bIIII_PPPPPPPPPPPP_IIII_EEEE_DDDDDDDD
        P: period
        I: instrument number
        E: effect number
        D: effect data
    */
    pub fn deserialize(input: u32) -> Self {
        let period = ((input >> 16) & 0x0FFF) as u16;
        let instrument_high = ((input >> 32 - 4) & 0x000F) as u8;
        let instrument_low = ((input >> 12) & 0x000F) as u8;
        let instrument = (instrument_high << 4) | instrument_low;
        let effect = ((input >> 8) & 0x000F) as u8;
        let data = (input & 0x00FF) as u8;

        // TODO: better error note handle any day
        let note = Self::period_to_note(period).unwrap_or(0);

        Self {
            note,
            instrument,
            effect,
            data,
        }
    }
}
