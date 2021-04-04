use std::{fs, io, path::Path};
use super::word::Word;
use rand::Rng;

pub fn parse<T: AsRef<Path>>(path: T, count: &u32, args: &[String]) -> Result<Vec<Word>, io::Error> {
    let file = fs::read_to_string(path);

    if let Err(err) = file {
        Err(err)
    } else {
        let file = file.unwrap();

        let chunks: Vec<&str> = file.split("\n\n").collect();

        let mut words: Vec<Word> = vec![];

        for _ in if *count > chunks.len() as u32 {
            0..chunks.len() as u32
        } else { 
            0..*count
        } {
            let rand = rand::thread_rng().gen_range(0..chunks.len());
            let word: Vec<&str> = chunks[rand].split('\n').collect();

            if !word[0].starts_with("#") {
                if args.iter().any(|arg| arg == &String::from("--no-defs")) {
                    words.push(Word::new(word[0], ""));
                } else {
                    words.push(Word::new(word[0], word[1]));
                }
            }
        }

        Ok(words)
    }
}
