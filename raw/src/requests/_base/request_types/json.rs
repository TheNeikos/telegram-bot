use crate::prelude::*;
use serde::Serialize;
use serde_json;

#[cfg(feature = "request_building")]
use crate::requests::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct JsonRequestType<Request> {
    phantom: marker::PhantomData<Request>,
}

impl<Request: Serialize> RequestType for JsonRequestType<Request> {
    type Options = RequestUrl;
    type Request = Request;

    fn serialize(url: Self::Options, request: &Self::Request) -> Result<HttpRequest, Error> {
        let body = serde_json::to_string(&request).map_err(ErrorKind::from)?;
        Ok(HttpRequest {
            url: url,
            method: Method::Post,
            body: Body::Json(body),
        })
    }
}
