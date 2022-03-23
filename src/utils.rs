use crate::interface::RenderError;
use crate::private::Sealed;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// A type that can never be (safly) initialized.
/// This is temporary, until
/// [`never`](https://doc.rust-lang.org/std/primitive.never.html)
/// is accepted into stable rust.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum Never {}

impl Display for Never {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Never {}

impl Sealed for Never {}

impl RenderError for Never {}
