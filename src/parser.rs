use super::{app::App, utils::usage};
use rand::Rng;
use serde_json::Value;
use std::{fs, io, path::Path};

pub struct Word {
    word: String,
    definition: String,
}

impl Word {
    pub fn new<T: ToString>(word: T, definition: T) -> Self {
        let word = word.to_string();
        let definition = definition.to_string();

        Self {
            word,
            definition,
        }
    }

    pub fn get_definition(&self) -> &String {
        &self.definition
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }
}

pub fn parse_words(args: &[String], app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    app.words = Vec::new();
    match &args[1][..] {
        "quote" => {
            let other_res = ureq::get("https://www.reddit.com/r/copypasta/top/.json?sort=top&t=week&showmedia=false&mediaonly=false&is_self=true&limit=100")
                .call()?.into_string()?;

            let json: Value = serde_json::from_str(&other_res[..]).unwrap();

            let quote = json["data"]["children"][rand::thread_rng().gen_range(0..100) as usize]["data"]["selftext"].as_str().unwrap();
            
            for word in quote.split(" ") {
                app.words.push(Word::new(word, ""));
            }
            Ok(())
        }

        _ => {
            if args.len() < 3 { usage(&args); }
            let count = args[2].parse::<u32>();

            if count.is_err() {
                usage(&args);
                std::process::exit(1);
            }

            let parsed_words = parse_wordlist(app.locate_wordlist(&args[1]), &count.unwrap(), &args);

            if let Err(err) = parsed_words {
                println!(
                    "\"{}\" is not a valid wordlist: {}",
                    &args[1],
                    err.to_string()
                );

                std::process::exit(1);
            }

            app.words = parsed_words.unwrap();
            Ok(())
        }
    }
}

pub fn parse_wordlist<T: AsRef<Path>>(path: T, count: &u32, args: &[String]) -> Result<Vec<Word>, io::Error> {
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
