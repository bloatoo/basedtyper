use rand::Rng;
use serde_json::Value;
use std::{fs, io, path::Path};
use std::io::Write;

pub fn calc_wpm(wordlist_len: f64, time_taken: f64) -> f64 {
    (wordlist_len / 5.0) / ((time_taken as f64 / 1000.0) / 60.0)
}


#[derive(Clone)]
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

pub fn parse_words(mode: &str, wordlist_path: Option<String>) -> Result<Vec<Word>, Box<dyn std::error::Error>> {
    match mode {
        "quote" => {
            let other_res = ureq::get("https://www.reddit.com/r/copypasta/top/.json?sort=top&t=week&showmedia=false&mediaonly=false&is_self=true&limit=100")
                .call()?.into_string()?;

            let json: Value = serde_json::from_str(&other_res[..]).unwrap();

            let mut words = Vec::new();

            let quote = json["data"]["children"][rand::thread_rng().gen_range(0..100) as usize]["data"]["selftext"].as_str().unwrap();

            let re = regex::Regex::new("[^a-zA-Z0-9\\d\\s\\.,':-]").unwrap();
            let re2 = regex::Regex::new("\\w[ ]{2,}\\w").unwrap();

            let quote = re.replace_all(quote, "").to_string();
            let quote = re2.replace_all(&quote[..], " ");
            let quote = quote.trim();
            
            for word in quote.split(' ') {
                words.push(Word::new(word, ""));
            }

            /*if app.config.cache_quotes {
                let home_dir = std::env::var("HOME").unwrap();

                let words_vec = app.wordlist.words();

                let word_string = words_vec.join(" ");


                /*if !Path::new(&format!("{}/.cache/basedtyper.cached_quotes", home_dir)).is_file() {
                    std::fs::File::create(format!("{}/.cache/basedtyper.cached_quotes", home_dir)).unwrap();
                }*/

                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(format!("{}/.cache/basedtyper.cached_quotes", home_dir))
                    .unwrap();

                file.write((word_string + "\n\n").as_bytes()).unwrap();
            }*/

            Ok(words)
        }

        _ => {
            let parsed_words = parse_wordlist(wordlist_path.unwrap(), &10);

            if let Ok(words) = parsed_words {
                return Ok(words);
            }

            Err(Box::new(parsed_words.err().unwrap()))
        }
    }
}

pub fn parse_wordlist<T: AsRef<Path>>(path: T, count: &u32) -> Result<Vec<Word>, io::Error> {
    let file = fs::read_to_string(path);

    if let Err(err) = file {
        Err(err)
    } else {
        let file = file.unwrap();

        let chunks: Vec<&str> = file.split("\n\n").collect();

        let mut words: Vec<Word> = vec![];

        for _ in if *count > chunks.len() as u32 {
            0..chunks.len() as u32 - 1
        } else { 
            0..*count
        } {
            let rand = rand::thread_rng().gen_range(0..chunks.len());
            let word: Vec<&str> = chunks[rand].split('\n').collect();

            words.push(Word::new(word[0], word[1]));
        }

        Ok(words)
    }
}

pub async fn generate_wordlist(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(file_path).unwrap();

    let mut f = std::fs::OpenOptions::new().append(true).create(true).open("wordlist.basedtyper").unwrap();

    for word in data.split(' ') {
        let url = format!("https://api.urbandictionary.com/v0/define?term={}&page=1", word);
        let res = ureq::get(&url[..]).call().unwrap().into_string().unwrap();

        let json: Value = serde_json::from_str(&res).unwrap();
        let arr = json["list"].as_array().unwrap();

        f.write_all(format!("{}\n{}\n\n", word.replace("\n", ""), arr[0]["definition"].as_str().unwrap()).as_bytes()).unwrap();
    }

    Ok(())
}
