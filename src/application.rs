use beryllium::*;
use crate::game::Game;

pub struct Application{
    sdl_handle:SDL,
    sdl_window:GlWindow,
    sdl_game:Game
}

impl Application {
    pub(crate) fn create_instance() -> Application {
        let sdl = Application::sdl_init();
        let window = Application::sdl_window(&sdl);


        return Application {
            sdl_handle: sdl,
            sdl_window: window,
            sdl_game: Game { }
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

    fn sdl_window(sdl: &SDL) -> GlWindow {
        let window = sdl
            .create_gl_window(
                "Hello Window",
                WindowPosition::Centered,
                800,
                600,
                WindowFlags::Shown,
            )
            .expect("couldn't make a window and context");

        return window;
    }

    pub fn init_game(&self){
        self.sdl_game.setup();

        'main_loop: loop {
            // handle events this frame
            while let Some(event) = self.sdl_handle.poll_events().and_then(Result::ok) {
                match event {
                    Event::Quit(_) => break 'main_loop,
                    _ => self.sdl_game.update_game(),
                }
            }
        }
    }


}