use toml;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Read};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

lazy_static! {
    static ref CONFIG_PATH: PathBuf = PathBuf::from("src/config/cnf.toml");
    static ref TOML: String = {
        let mut _config= fs::File::open(&*CONFIG_PATH).expect(&format!("Unable to load {}", CONFIG_PATH.display()));
        let mut contents = String::new();
        _config.read_to_string(&mut contents)
            .expect(&format!("Failed to parse {}", CONFIG_PATH.display()));
        contents
    };
}

#[derive(Serialize, Deserialize)]
pub struct ColorScheme {
    pub bg: String,
    pub fg: String
}

#[derive(Serialize, Deserialize)]
pub struct ConfigOptions {
    pub ops: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct ConfigModes {
    pub normal: String,
    pub shell: String,
    pub options: String,
    pub end: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigCommands {
    pub end: String,
    pub exp: String,
    pub ser: String,
    pub scd: String,
    pub del: String,
    pub cpy: String,
    pub opn: String,
    pub mov: String
}


#[derive(Serialize, Deserialize)]
pub struct Config {
    pub color_schemes: ColorScheme,
    pub modes: ConfigModes,
    pub commands: ConfigCommands
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let config: Config = toml::from_str(&TOML.as_str())
            .expect(&format!("Failed to serialize from {}", CONFIG_PATH.display()));
        Ok(config)
    }
}

/*
end = "quits from the application."
exp = "explore everything inside cwd."
ser = "searches for a given file inside cwd."
scd = "switches the cwd to the given directory."
del = "deletes a given file and moves it to recycle bin."
cpy = "copies a given file."
opn = "opens the contents of a given file."
mov = "moves the given file to a given path." 
*/