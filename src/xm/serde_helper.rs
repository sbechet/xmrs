use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};

use alloc::string::String;
use alloc::string::ToString;

// --- deserialize -------------------------

macro_rules! make_deserialize_string_fn {
    ($name:ident, $limit:expr) => {
        pub fn $name<'de, D>(deserializer: D) -> Result<String, D::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes = <[u8; $limit]>::deserialize(deserializer)?;
            let s = String::from_utf8_lossy(&bytes).to_string();
            let s = s.trim_matches(char::from(0)).trim().to_string(); // cleanup
            Ok(s)
        }
    };
}

make_deserialize_string_fn!(deserialize_string_17, 17);
make_deserialize_string_fn!(deserialize_string_20, 20);
make_deserialize_string_fn!(deserialize_string_21, 21);
make_deserialize_string_fn!(deserialize_string_22, 22);

// --- serialize ---------------------------

macro_rules! make_serialize_string_fn {
    ($name:ident, $limit:expr) => {
        pub fn $name<S>(value: &String, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let bytes = value.as_bytes();
            let mut count = 0;
            let mut i = 0;
            while i < bytes.len() && count < $limit {
                let ch = bytes[i];
                let width = utf8_char_width(ch);
                if count + width > $limit {
                    break;
                }
                count += width;
                i += width;
            }
            if i < bytes.len() && utf8_char_width(bytes[i]) > 1 {
                while i > 0 && utf8_char_width(bytes[i - 1]) > 1 {
                    i -= 1;
                }
            }
            let mut array = [0u8; $limit];
            array[..i].copy_from_slice(&bytes[..i]);
            array.serialize(serializer)
        }
    };
}

fn utf8_char_width(ch: u8) -> usize {
    match ch {
        0..=127 => 1,
        128..=191 => 0,
        192..=223 => 2,
        224..=239 => 3,
        240..=247 => 4,
        248..=255 => 0,
    }
}

make_serialize_string_fn!(serialize_string_17, 17);
make_serialize_string_fn!(serialize_string_20, 20);
make_serialize_string_fn!(serialize_string_21, 21);
make_serialize_string_fn!(serialize_string_22, 22);
