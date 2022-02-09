use std::{path::PathBuf, fs};

use dirs::home_dir;


pub fn get_config_dir() -> PathBuf {
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