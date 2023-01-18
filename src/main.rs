mod application;
mod game;

use application::*;

fn main() {
    let app = Application::create_instance();
    app.init_game();
}
