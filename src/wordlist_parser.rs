use std::io;
use std::{fs, path::Path};

pub fn parse<T: AsRef<Path>>(path: T, args: &Vec<String>) -> Result<Vec<(String, String)>, io::Error> {
    let file = fs::read_to_string(path);

    if let Err(err) = file {
        return Err(err);
    } else {
        let file = file.unwrap();

        let chunks: Vec<&str> = file.split("\n\n").collect();

        let mut words: Vec<(String, String)> = vec![];

        chunks.iter().for_each(|chunk| {
            let word: Vec<&str> = chunk.split("\n").collect();
            if let Some(_) = args.iter().find(|arg| arg == &&String::from("--no-defs")) {
                words.push((String::from(word[0]), String::new()));
            } else {
                words.push((String::from(word[0]), String::from(word[1])));
            }
        });

        return Ok(words);
    }
}
