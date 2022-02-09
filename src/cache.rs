use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::{util, github};

const CACHE_FILE_NAME: &str = "cache.json";


#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub timestamp: u64,
    pub data: Vec<github::Repo>,
}

pub fn get_filepath() -> PathBuf {
    let mut config_path = util::get_config_dir();
    config_path.push(CACHE_FILE_NAME);
    config_path
}

pub fn read() -> Option<Cache> {
    let path = get_filepath();

    if !path.as_path().exists() {
        return None;
    }

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let cache: Cache = serde_json::from_str(&contents).unwrap();

    Some(cache)
}

fn write_cache(path: PathBuf, data: Cache) {
    let mut file = File::create(path).unwrap();
    let json = serde_json::to_string(&data).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn write(data: Vec<github::Repo>) {
    let cache = Cache {
        timestamp: util::get_timestamp(),
        data,
    };

    let path = get_filepath();
    write_cache(path, cache);
}
