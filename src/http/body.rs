//! HTTP body types

use crate::io::{AsyncRead, Cursor};

/// A trait representing an HTTP body.
pub trait Body: AsyncRead {
    /// Returns the exact remaining length of the iterator, if known.
    fn len(&self) -> Option<usize>;

    /// Returns `true`` if the body is known to be empty.
    fn is_empty(&self) -> bool {
        matches!(self.len(), Some(0))
    }
}

/// Conversion into a `Body`.
pub trait IntoBody {
    /// What type of `Body` are we turning this into?
    type IntoBody: Body;
    /// Convert into `Body`.
    fn into_body(self) -> Self::IntoBody;
}
impl<T> IntoBody for T
where
    T: Body,
{
    type IntoBody = T;
    fn into_body(self) -> Self::IntoBody {
        self
    }
}

impl<T> Body for T
where
    T: AsyncRead,
{
    fn len(&self) -> Option<usize> {
        None
    }
}

/// An HTTP body with a known length
#[derive(Debug)]
pub struct BoundedBody<T>(Cursor<T>);

impl<T: AsRef<[u8]>> AsyncRead for BoundedBody<T> {
    async fn read(&mut self, buf: &mut [u8]) -> crate::io::Result<usize> {
        self.0.read(buf).await
    }
}