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
    pub use alloc::borrow::Cow;
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::fmt;
    pub use alloc::format;
    pub use alloc::str;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec::Vec;
    pub use core::marker;
    pub use core::ops::Deref;
    pub use core::ops::Not;
}

#[cfg(not(feature = "no_std"))]
mod prelude {
    pub use std::borrow::Cow;
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
    pub use std::fmt;
    pub use std::format;
    pub use std::marker;
    pub use std::ops::Deref;
    pub use std::ops::Not;
    pub use std::str;
    pub use std::string::String;
    pub use std::string::ToString;
    pub use std::vec::Vec;
}

#[cfg(feature = "request_building")]
pub use crate::requests::*;
pub use crate::types::*;
pub use crate::url::*;
