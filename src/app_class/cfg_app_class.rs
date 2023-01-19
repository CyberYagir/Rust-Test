use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use beryllium::WindowPosition;
use serde::{Serialize, Deserialize};
use crate::app_class::vector_class::Vector2;

#[derive(Serialize, Deserialize, Debug)]
pub enum WindowType{
    Centered,
    Undefined,
    Position
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowPos{
    display_type: WindowType,
    position: Vector2<i32>,
    size: Vector2<u32>,
}
impl WindowPos{
    pub fn get_gl_state(&self) -> WindowPosition {

        return  match self.display_type {
            WindowType::Centered | _ => WindowPosition::Centered,
            WindowType::Undefined => WindowPosition::Undefined,
            WindowType::Position => WindowPosition::XY(self.position.x, self.position.y)
        }
    }

    pub fn get_size(&self) -> Vector2<u32> {
        let size: Vector2<u32> = Vector2 {
            x: self.size.x,
            y: self.size.y,
        };
        return size;
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub window_data: WindowPos,
    pub window_name: String,
}

impl AppConfig {
    pub fn load_data() -> AppConfig {
        let mut data: AppConfig = AppConfig {
            window_data: WindowPos {
                display_type: WindowType::Centered,
                position: Default::default(),
                size: Vector2{
                    x:800,
                    y:600
                },
            },
            window_name: "RustEngine".to_string(),
        };

        match env::current_exe() {
            Ok(exe_path) => {
                let root_folder = exe_path.parent().unwrap();
                let options_path_str: String = root_folder.to_str().unwrap().to_string() + "\\boot.cfg";
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
                    let json = serde_json::to_string(&data).unwrap();
                    file.write_all(json.as_bytes());
                }
            }
            Err(e) => println!("failed to get current exe path: {e}"),
        };


        return data;
    }
}
