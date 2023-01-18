use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use beryllium::WindowPosition;
use ogl33::GLuint;
use serde::{Serialize, Deserialize};
use crate::app_class::vector_class::Vector2;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct WindowPos{
    display_type: String,
    size: Vector2<i32>,
}
impl WindowPos{
    fn get_gl_state(&self) -> WindowPosition {

        return  match self.display_type.as_str() {
            "Centered" => WindowPosition::Centered,
            "Undefined" => WindowPosition::Undefined,
            "Position" => WindowPosition::XY(*self.size.X(), *self.size.Y()),
            _ => WindowPosition::Centered,
        }
    }
}



#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AppConfig {
    window_pos: WindowPos,
    window_name: String,
}

impl AppConfig {
    pub fn load_data() -> AppConfig {
        let mut data: AppConfig = AppConfig::default();

        match env::current_exe() {
            Ok(exe_path) => {
                let root_folder = exe_path.parent().unwrap();
                let mut options_path_str: String = root_folder.to_str().unwrap().to_string();
                options_path_str.push_str("\\boot.cfg");


                let file_path = Path::new(options_path_str.as_str());


                if file_path.exists() {
                    let json = fs::read_to_string(file_path).unwrap();
                    let result = serde_json::from_slice(json.as_bytes());

                    match result {
                        Result::Ok(val) => { data = val },
                        Result::Err(err) => {
                             fs::remove_file(file_path);
                             data = AppConfig::load_data();
                        }
                    }
                } else {
                    let mut file = File::create(file_path).expect("Error encountered while creating file!");
                    data = AppConfig {
                        window_pos: WindowPos {
                            display_type: "Centered".to_string(),
                            size: Default::default(),
                        },
                        window_name: "RustEngine".to_string(),
                    };
                    let json = serde_json::to_string(&data).unwrap();
                    file.write_all(json.as_bytes());
                }
            }
            Err(e) => println!("failed to get current exe path: {e}"),
        };


        return data;
    }
}
