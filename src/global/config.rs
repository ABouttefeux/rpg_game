use console::Color;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

type NumConsole = usize;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Configuration {
    width: NumConsole,
    height: NumConsole,
    //color: Color,
}

impl Configuration {
    pub const fn new(width: NumConsole, height: NumConsole) -> Self {
        Self { width, height }
    }

    pub const fn width(&self) -> NumConsole {
        self.width
    }

    pub const fn height(&self) -> NumConsole {
        self.height
    }

    pub const fn new_width(&self, number: NumConsole) -> Option<Width<'_>> {
        if number < self.width {
            Some(Width {
                width: number,
                config: self,
            })
        } else {
            None
        }
    }

    pub const fn new_height(&self, number: NumConsole) -> Option<Height<'_>> {
        if number < self.width {
            Some(Height {
                height: number,
                config: self,
            })
        } else {
            None
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            width: 100,
            height: 40, // TODO review numbers
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize))]
pub struct Height<'a> {
    height: NumConsole,
    config: &'a Configuration,
}

impl<'a> Height<'a> {
    pub const fn height(&self) -> NumConsole {
        self.height
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize))]
pub struct Width<'a> {
    width: NumConsole,
    config: &'a Configuration,
}

impl<'a> Width<'a> {
    pub const fn width(&self) -> NumConsole {
        self.width
    }
}
