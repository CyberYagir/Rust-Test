mod app_class;
mod game_base_class;
mod shaders_list;
mod gl_methods;

use app_class::*;

fn main() {


    let mut app = App::create_instance();
    app.init_game();
}
