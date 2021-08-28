extern crate dirs;

use std::path::{Path, PathBuf};
use std::fs::{File, read_to_string};

use dirs::config_dir;

use super::super::structs::classes::Classes;

pub fn config_file() -> PathBuf {
    config_dir().unwrap().join("skid")
}

pub fn config_exists() -> bool {
    Path::new(&config_file()).exists()
}

pub fn create_config() {
    match File::create(config_file()) {
        Ok(_) => (),
        Err(e) => println!("Could not create config file: {}", e)
    }
}

pub fn write_config(classes: &Classes) {
    classes.write(config_file());
}

pub fn read_config() -> Classes {
    match read_to_string(config_file()) {
        Ok(s) => Classes::parse(s),
        Err(e) => {
            println!("Could not read config file: {}\nDefaulting to blank data", e);
            Classes::new()
        }
    } 
}