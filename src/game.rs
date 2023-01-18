use beryllium::*;
use crate::application::Application;

mod setup_logic;
mod update_logic;

pub struct Game{}
impl Game {
    pub fn setup(&self) {
        setup_logic::setup_logic(&self)
    }

    pub fn update_game(&self) {
        update_logic::update_logic(&self)
    }
}
