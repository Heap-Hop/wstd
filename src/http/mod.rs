//! HTTP networking support
//!
pub use http::uri::Uri;

#[doc(inline)]
pub use body::{Body, IntoBody};
pub use client::Client;
pub use error::{Error, Result};
pub use fields::{FieldName, FieldValue, Fields, Headers, Trailers};
pub use method::Method;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

pub(crate) use fields::fields_from_wasi;

pub mod body;

mod client;
mod error;
mod fields;
mod method;
mod request;
mod response;
mod status_code;
