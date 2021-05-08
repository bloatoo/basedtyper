use std::{fs, env, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub multiplayer: MultiplayerOptions,
    pub general: GeneralOptions,
}

#[derive(Deserialize)]
pub struct MultiplayerOptions {
    pub username: String,
}

#[derive(Deserialize)]
pub struct GeneralOptions {
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

        let data: Result<Config, _> = toml::from_str(&file_contents.unwrap());

        if let Err(_err) = data {
            return Ok(Config::default());
        }

        Ok(data.unwrap())
    }

    pub fn default() -> Self {
        let home_dir = env::var("HOME").unwrap();
        let default_path = format!("{}/.local/share/basedtyper", home_dir);

        if !Path::new(&default_path).is_dir() {
            fs::create_dir(&default_path).unwrap();
            fs::create_dir(default_path.clone() + "/wordlists").unwrap();
        }

        Self {
            multiplayer: MultiplayerOptions {
                username: String::from("anonymous"),
            }, 

            general: GeneralOptions {
                wordlist_directory: default_path + "/wordlists",
                definitions: true,
                cache_quotes: false,
            }
        }
    }

}
