use super::Interface;
use crate::{Never, Terminal};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Map {}

impl Map {}

impl Interface for Map {
    type Error = Never;
    fn render(&mut self, terminal: &Terminal) -> Result<(), Self::Error> {
        todo!()
    }
}
