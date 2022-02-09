use std::{fs::{File, self}, path::{PathBuf}, io::{Write, Read}};
use serde::{Serialize, Deserialize};
use dirs::home_dir;

const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token: String,
    pub username: String,
}

pub fn get_dir() -> PathBuf {
    let mut config_path = match home_dir() {
        Some(path) => path,
        None => {
            panic!("Could not find home directory");
        }
    };

    config_path.push(".ghost/");

    if fs::create_dir_all(&config_path).is_err() {
        panic!("Failed to create config directory");
    }

    config_path
}

pub fn get_filepath() -> PathBuf {
    let mut config_path = get_dir();
    config_path.push(CONFIG_FILE_NAME);
    config_path
}

pub fn read() -> Option<Config> {
    let path = get_filepath();

    if !path.as_path().exists() {
        return None;
    }

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = serde_json::from_str(&contents).unwrap();
    Some(config)
}

pub fn write(config: Config) {
    let path = get_filepath();
    let mut file = File::create(path).unwrap();
    let json = serde_json::to_string(&config).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
