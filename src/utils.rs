pub fn calc_wpm(wordlist_len: f64, time_taken: f64) -> f64 {
    (wordlist_len / 5.0) / ((time_taken as f64 / 1000.0) / 60.0)
}
