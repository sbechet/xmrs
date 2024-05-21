use serde::Deserialize;
use serde::Deserializer;

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
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
