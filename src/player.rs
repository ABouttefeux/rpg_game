use crate::basic::{Healt, Level, Mana, Stats};
use crate::object::Object;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Player {
    name: String,
    class: Class,
    level: Level,
    healt: Healt,
    mana: Mana,
    stats: Stats,
    //inventory: Vec<Box<dyn Object>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))] //#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]

pub enum Class {}
