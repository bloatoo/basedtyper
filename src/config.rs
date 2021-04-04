use std::{fs, env, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub wordlist_directory: String,
}

impl Config {
    pub fn new() -> Result<Self, std::io::Error> {
        let file_contents = std::fs::read_to_string(".config/basedtyper/config.toml");

        if let Err(err) = file_contents {
            return Err(err);
        }

        let data: Config = toml::from_str(&file_contents.unwrap()).unwrap();

        Ok(Self {
            wordlist_directory: data.wordlist_directory,
        })
    }

    pub fn default() -> Self {
        let home_dir = env::var("HOME").unwrap();
        let default_path = format!("{}/.local/share/basedtyper", home_dir);

        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
            fs::create_dir(default_path.clone() + "/wordlists").unwrap();
        }

        Self {
            wordlist_directory: default_path + "/wordlists" 
        }
    }

}
