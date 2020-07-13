use crate::prelude::*;

#[cfg(not(feature = "no_std"))]
use std::path::Path;

use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn serialize_bytes<S>(input: &Bytes, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.collect_seq(&input[..])
}

fn deserialize_bytes<'de, D>(des: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Vec::<u8>::deserialize(des)?;
    Ok(buf.into())
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct Text(
    #[serde(serialize_with = "serialize_bytes")]
    #[serde(deserialize_with = "deserialize_bytes")]
    Bytes,
);

impl Text {
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text(value.into())
    }
}

impl<'a> From<&'a str> for Text {
    fn from(value: &'a str) -> Self {
        Text(value.to_owned().into())
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(not(feature = "no_std"))]
impl AsRef<Path> for Text {
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}
