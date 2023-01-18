mod application;
use application::*;

fn main() {
    let app = Application::create_instance();
    app.app_update();
}
