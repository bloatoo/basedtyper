use std::{fs, env, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub wordlist_directory: String,
    pub definitions: bool,
    pub cache_quotes: bool,
}

impl Config {
    pub fn new() -> Result<Self, std::io::Error> {
        let home_dir = env::var("HOME").unwrap();

        let file_contents = std::fs::read_to_string(&format!("{}/.config/basedtyper/config.toml", home_dir)[..]);

        if let Err(err) = file_contents {
            return Err(err);
        }

        let data: Result<Self, _> = toml::from_str(&file_contents.unwrap());

        if let Err(_err) = data {
            return Ok(Config::default());
        }

        let data = data.unwrap();

        Ok(Self {
            wordlist_directory: data.wordlist_directory,
            definitions: data.definitions,
            cache_quotes: data.cache_quotes,
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
            wordlist_directory: default_path + "/wordlists",
            definitions: true,
            cache_quotes: false,
        }
    }

}
