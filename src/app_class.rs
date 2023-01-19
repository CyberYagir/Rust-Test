use beryllium::*;
use ogl33::*;

use std::time::Instant;


use crate::game_base_class::Game;
use crate::app_class::cfg_app_class::AppConfig;

mod cfg_app_class;
mod vector_class;

pub struct App {
    sdl_handle:SDL,
    sdl_window:GlWindow,
    sdl_game:Game,

    app_config: AppConfig,

    timestamp: Instant,
    delta_time:f64
}

impl App {
    pub(crate) fn create_instance() -> App {
        let sdl = App::sdl_init();
        let cfg = AppConfig::load_data();
        let window = App::sdl_window(&sdl, &cfg);


        return App {
            sdl_handle: sdl,
            sdl_window: window,
            sdl_game: Game::default(),

            app_config: cfg,

            delta_time: 0.0,
            timestamp: Instant::now()
        }
    }

    fn sdl_init() -> SDL {
        let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
        sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
        sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
        sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
        #[cfg(target_os = "macos")]
        {
            sdl
                .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
                .unwrap();
        }
        return sdl;
    }

    fn sdl_window(sdl: &SDL, cfg: &AppConfig) -> GlWindow {
        let title = cfg.window_name.as_str();
        let pos = cfg.window_data.get_gl_state();
        let size = cfg.window_data.get_size();
        let window = sdl
            .create_gl_window(
                title,
                pos,
                size.x,
                size.y,
                beryllium::WindowFlags::Resizable,
            )
            .expect("couldn't make a window and context");
        return window;
    }

    pub fn init_game(&mut self){
        unsafe {
            self.sdl_game.setup_gl(&self.sdl_window);
            self.sdl_game.setup();


            'main_loop: loop {
                while let Some(event) = self.sdl_handle.poll_events().and_then(Result::ok) {
                    match event {
                        Event::Quit(_) => break 'main_loop,
                        _ => (),
                    }
                }


                self.delta_time = self.timestamp.elapsed().as_secs_f64();
                self.timestamp = Instant::now();

                glClear(GL_COLOR_BUFFER_BIT);
                self.sdl_game.update_game(self.delta_time);

                self.sdl_window.swap_window();
            }
        }
    }


}