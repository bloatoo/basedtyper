use serde_json::Value;
use std::io::Write;

pub async fn generate_wordlist(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(file_path).unwrap();

    let mut f = std::fs::OpenOptions::new().append(true).create(true).open("wordlist.basedtyper").unwrap();

    for word in data.split(" ") {
        let url = format!("https://api.urbandictionary.com/v0/define?term={}&page=1", word);
        let res = ureq::get(&url[..]).call().unwrap().into_string().unwrap();

        let json: Value = serde_json::from_str(&res).unwrap();
        let arr = json["list"].as_array().unwrap();

        f.write(format!("{}\n{}\n\n", word.replace("\n", ""), arr[0]["definition"].as_str().unwrap()).as_bytes()).unwrap();
    }

    Ok(())
}
