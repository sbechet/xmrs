use serde::Deserialize;
use serde::Deserializer;

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

make_deserialize_string_fn!(deserialize_string_4, 4);
make_deserialize_string_fn!(deserialize_string_12, 12);
make_deserialize_string_fn!(deserialize_string_28, 28);
