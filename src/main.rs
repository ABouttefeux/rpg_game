use rpg_game::global::{Configuration, Terminal};
use rpg_game::interface::{Interface, MainMenu};

fn main() {
    let mut main_menu = MainMenu::new();
    let terminal = Terminal::new(Configuration::default());
    main_menu.render(&terminal).unwrap();
}
