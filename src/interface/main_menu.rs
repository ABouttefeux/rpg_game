use super::{Interface, RenderErrorEnum};
use crate::Terminal;
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct MainMenu {
    choice: MainMenuChoice,
    need_to_clear: bool,
}

impl MainMenu {
    pub const fn new() -> Self {
        Self {
            choice: MainMenuChoice::NewGame,
            need_to_clear: true,
        }
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum MainMenuChoice {
    /// New Game
    NewGame,
    /// Load Game
    LoadGame,
    /// Option
    Options,
    /// Exit.
    Exit,
}

impl MainMenuChoice {
    pub const fn choices() -> [MainMenuChoice; 4] {
        [
            MainMenuChoice::NewGame,
            MainMenuChoice::LoadGame,
            MainMenuChoice::Options,
            MainMenuChoice::Exit,
        ]
    }
}

impl Default for MainMenuChoice {
    fn default() -> Self {
        MainMenuChoice::NewGame
    }
}

impl Display for MainMenuChoice {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            MainMenuChoice::NewGame => write!(f, "New Game"),
            MainMenuChoice::LoadGame => write!(f, "Load Game"),
            MainMenuChoice::Options => write!(f, "Options"),
            MainMenuChoice::Exit => write!(f, "Exit"),
        }
    }
}

impl Interface for MainMenu {
    type Error = RenderErrorEnum;
    fn render(&mut self, terminal: &Terminal) -> Result<(), Self::Error> {
        if self.need_to_clear {
            terminal.term().clear_screen()?;
            self.need_to_clear = false;
        }
        for (i, choice) in MainMenuChoice::choices().iter().enumerate() {
            terminal.write_text_centered(
                &format!("{}", choice),
                terminal.config().new_height(i * 2 + 8).unwrap(),
            )?;
        }
        Ok(())
    }
}
