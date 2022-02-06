use std::{fs::File, path::{PathBuf}, io::{stdin, stdout, Write, Read}};
use serde::{Serialize, Deserialize};
use dirs::home_dir;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub username: String,
}

pub fn get_filepath() -> PathBuf {
    let mut config_path = match home_dir() {
        Some(path) => path,
        None => {
            let mut input = String::new();
            print!("Please enter a directory to save the config file: ");
            stdout().flush().unwrap_or_default();
            stdin().read_line(&mut input).expect("Please enter a directory to save the config file: ");
            PathBuf::from(input)
        }
    };

    config_path.push(".ghostconfig");

    config_path
}

pub fn read(path: PathBuf) -> Config {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = serde_json::from_str(&contents).unwrap();
    config
}

pub fn write(path: PathBuf, config: Config)  {
    let mut file = File::create(path).unwrap();
    let json = serde_json::to_string(&config).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}