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
            6848 => Some(1),
            6464 => Some(2),
            6096 => Some(3),
            5760 => Some(4),
            5424 => Some(5),
            5120 => Some(6),
            4832 => Some(7),
            4560 => Some(8),
            4304 => Some(9),
            4064 => Some(10),
            3840 => Some(11),
            3624 => Some(12),
            3424 => Some(13),
            3232 => Some(14),
            3048 => Some(15),
            2880 => Some(16),
            2712 => Some(17),
            2560 => Some(18),
            2416 => Some(19),
            2280 => Some(20),
            2152 => Some(21),
            2032 => Some(22),
            1920 => Some(23),
            1812 => Some(24),
            1712 => Some(25),
            1616 => Some(26),
            1524 => Some(27),
            1440 => Some(28),
            1356 => Some(29),
            1280 => Some(30),
            1208 => Some(31),
            1140 => Some(32),
            1076 => Some(33),
            1016 => Some(34),
            960 => Some(35),
            906 => Some(36),
            856 => Some(37),
            808 => Some(38),
            762 => Some(39),
            720 => Some(40),
            678 => Some(41),
            640 => Some(42),
            604 => Some(43),
            570 => Some(44),
            538 => Some(45),
            508 => Some(46),
            480 => Some(47),
            453 => Some(48),
            428 => Some(49),
            404 => Some(50),
            381 => Some(51),
            360 => Some(52),
            339 => Some(53),
            320 => Some(54),
            302 => Some(55),
            285 => Some(56),
            269 => Some(57),
            254 => Some(58),
            240 => Some(59),
            226 => Some(60),
            214 => Some(61),
            202 => Some(62),
            190 => Some(63),
            180 => Some(64),
            170 => Some(65),
            160 => Some(66),
            151 => Some(67),
            143 => Some(68),
            135 => Some(69),
            127 => Some(70),
            120 => Some(71),
            113 => Some(72),
            107 => Some(73),
            101 => Some(74),
            95 => Some(75),
            90 => Some(76),
            85 => Some(77),
            80 => Some(78),
            75 => Some(79),
            71 => Some(80),
            67 => Some(81),
            63 => Some(82),
            60 => Some(83),
            56 => Some(84),
            0 => Some(0),
            _ => None,
        }
    }

    fn note_to_text(note: u8) -> Option<&'static str> {
        match note {
            0 => Some("---"),
            1 => Some("C-2"),
            2 => Some("C#2"),
            3 => Some("D-2"),
            4 => Some("D#2"),
            5 => Some("E-2"),
            6 => Some("F-2"),
            7 => Some("F#2"),
            8 => Some("G-2"),
            9 => Some("G#2"),
            10 => Some("A-2"),
            11 => Some("A#2"),
            12 => Some("B-2"),
            13 => Some("C-3"),
            14 => Some("C#3"),
            15 => Some("D-3"),
            16 => Some("D#3"),
            17 => Some("E-3"),
            18 => Some("F-3"),
            19 => Some("F#3"),
            20 => Some("G-3"),
            21 => Some("G#3"),
            22 => Some("A-3"),
            23 => Some("A#3"),
            24 => Some("B-3"),

            25 => Some("C-4"),
            26 => Some("C#4"),
            27 => Some("D-4"),
            28 => Some("D#4"),
            29 => Some("E-4"),
            30 => Some("F-4"),
            31 => Some("F#4"),
            32 => Some("G-4"),
            33 => Some("G#4"),
            34 => Some("A-4"),
            35 => Some("A#4"),
            36 => Some("B-4"),
            37 => Some("C-5"),
            38 => Some("C#5"),
            39 => Some("D-5"),
            40 => Some("D#5"),
            41 => Some("E-5"),
            42 => Some("F-5"),
            43 => Some("F#5"),
            44 => Some("G-5"),
            45 => Some("G#5"),
            46 => Some("A-5"),
            47 => Some("A#5"),
            48 => Some("B-5"),
            49 => Some("C-6"),
            50 => Some("C#6"),
            51 => Some("D-6"),
            52 => Some("D#6"),
            53 => Some("E-6"),
            54 => Some("F-6"),
            55 => Some("F#6"),
            56 => Some("G-6"),
            57 => Some("G#6"),
            58 => Some("A-6"),
            59 => Some("A#6"),
            60 => Some("B-6"),
            61 => Some("C-7"),
            62 => Some("C#7"),
            63 => Some("D-7"),
            64 => Some("D#7"),
            65 => Some("E-7"),
            66 => Some("F-7"),
            67 => Some("F#7"),
            68 => Some("G-7"),
            69 => Some("G#7"),
            70 => Some("A-7"),
            71 => Some("A#7"),
            72 => Some("B-7"),
            73 => Some("C-8"),
            74 => Some("C#8"),
            75 => Some("D-8"),
            76 => Some("D#8"),
            77 => Some("E-8"),
            78 => Some("F-8"),
            79 => Some("F#8"),
            80 => Some("G-8"),
            81 => Some("G#8"),
            82 => Some("A-8"),
            83 => Some("A#8"),
            84 => Some("B-8"),
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
        let note = match Self::period_to_note(period) {
            Some(n) => n,
            None => {
                println!("Amiga Module Strange Period: {}?", period);
                0
            }
        };

        Self {
            note,
            instrument,
            effect,
            data,
        }
    }
}
