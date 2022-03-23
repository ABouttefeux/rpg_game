pub mod config;

pub use config::{Configuration, Height, Width};

use console::Term;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;

#[derive(Debug, Clone)]
//#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Terminal {
    config: Configuration,
    term: Term,
}

impl Terminal {
    pub fn new(config: Configuration) -> Self {
        Terminal {
            config,
            term: Term::stdout(),
        }
    }

    pub const fn config(&self) -> &Configuration {
        &self.config
    }

    pub const fn term(&self) -> &Term {
        &self.term
    }

    pub fn clear(&mut self) -> Result<(), IoError> {
        self.term.clear_screen()
    }

    pub fn write_text_centered(&self, text: &str, pos: Height<'_>) -> Result<(), IoError> {
        self.term
            .move_cursor_to((self.config.width() / 2) - (text.len() / 2), pos.height())?;
        self.term.write_line(text)
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            config: Configuration::default(),
            term: Term::stdout(),
        }
    }
}
