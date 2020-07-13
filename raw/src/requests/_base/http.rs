use crate::prelude::*;

use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::Text;
use crate::url::telegram_api_url;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum RequestUrl {
    Method(String),
}

impl RequestUrl {
    pub fn method(method: &'static str) -> Self {
        RequestUrl::Method(method.into())
    }

    #[cfg(not(feature = "no_std"))]
    pub fn url(&self, token: &str) -> String {
        match self {
            &RequestUrl::Method(method) => format!("{}bot{}/{}", telegram_api_url(), token, method),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
}

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum MultipartValue {
    Text(Text),
    Path {
        path: Text,
        file_name: Option<Text>,
    },
    Data {
        file_name: Text,
        #[serde(serialize_with = "serialize_bytes")]
        #[serde(deserialize_with = "deserialize_bytes")]
        data: Bytes,
    },
}

pub type Multipart = Vec<(String, MultipartValue)>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub enum Body {
    Empty,
    Multipart(Multipart),
    Json(String),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Body::Empty => "<empty body>".fmt(f),
            Body::Multipart(multipart) => write!(f, "{:?}", multipart),
            Body::Json(s) => s.fmt(f),
            Body::__Nonexhaustive => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct HttpRequest {
    pub url: RequestUrl,
    pub method: Method,
    pub body: Body,
}

impl HttpRequest {
    pub fn name(&self) -> &str {
        match &self.url {
            RequestUrl::Method(method) => &method,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct HttpResponse {
    pub body: Option<Vec<u8>>,
}
