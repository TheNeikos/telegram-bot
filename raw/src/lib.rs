#![cfg_attr(feature = "no_std", no_std)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "no_std")]
extern crate alloc;

pub mod requests;
pub mod types;
pub mod url;

#[cfg(feature = "no_std")]
mod prelude {
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::str;
    pub use alloc::vec::Vec;
    pub use core::ops::Deref;
    pub use alloc::boxed::Box;
    pub use alloc::borrow::Cow;
    pub use alloc::borrow::ToOwned;
    pub use core::ops::Not;
    pub use alloc::fmt;
    pub use alloc::format;
}

#[cfg(not(feature = "no_std"))]
mod prelude {
    pub use std::string::String;
    pub use std::vec::Vec;
    pub use std::ops::Deref;
    pub use std::boxed::Box;
    pub use std::borrow::Cow;
    pub use std::ops::Not;
}

#[cfg(feature = "request_building")]
pub use crate::requests::*;
pub use crate::types::*;
pub use crate::url::*;
