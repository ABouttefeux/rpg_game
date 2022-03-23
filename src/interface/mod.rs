mod combat;
mod main_menu;
mod map;

pub use combat::*;
pub use main_menu::*;
pub use map::*;

use crate::global::Terminal;
use crate::private::Sealed;
use crate::utils::Never;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;

//type Result<T> = std::result::Result<T, RenderErrorEnum>;

#[derive(Debug)]
pub enum RenderErrorEnum {
    IoError(IoError),
}

impl From<IoError> for RenderErrorEnum {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

pub trait RenderError: Sealed {}

impl Sealed for RenderErrorEnum {}

impl RenderError for RenderErrorEnum {}

pub trait Interface {
    type Error: RenderError;
    fn render(&mut self, terminal: &Terminal) -> Result<(), Self::Error>;
}
