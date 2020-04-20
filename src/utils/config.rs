use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone)]
pub struct Config {
    file: String,
    json_data: json::JsonValue
}

impl Config {
    pub fn new(file_path: &str) -> Config {
        let mut file = File::open(file_path).expect("[Config] Config non trouvée");
        let mut data = String::new();
        let _res = file.read_to_string(&mut data);
        let json_data = json::parse(&*data).expect("[Config] Config mal formée");
        Config {
            file: file_path.to_owned(),
            json_data: json_data
        }
    }

    pub fn save(&mut self) {
        let mut file = File::create(&*self.file).unwrap();
        match file.write_all(self.json_data.dump().as_bytes()) {
            Err(e) => println!("{}", e),
            Ok(_) => match file.flush() {
                Err(e) => println!("{}", e),
                Ok(_) => ()
            }
        }
    }

    pub fn get(&self, value: &str) -> &json::JsonValue {
        &self.json_data[value]
    }

    pub fn set_bool(&mut self, value: &str, val: bool) {
        self.json_data[value] = json::parse(&*json::stringify(val)).unwrap();
    }
}