use std::io;
use std::{fs, path::Path};

use super::word::Word;

pub fn parse<T: AsRef<Path>>(path: T, args: &[String]) -> Result<Vec<Word>, io::Error> {
    let file = fs::read_to_string(path);

    if let Err(err) = file {
        Err(err)
    } else {
        let file = file.unwrap();

        let chunks: Vec<&str> = file.split("\n\n").collect();

        let mut words: Vec<Word> = vec![];

        chunks.iter().for_each(|chunk| {
            let word: Vec<&str> = chunk.split('\n').collect();

            if args.iter().any(|arg| arg == &String::from("--no-defs")) {
                words.push(Word::new(word[0], ""));
            } else {
                words.push(Word::new(word[0], word[1]));
            }
        });

        Ok(words)
    }
}
