pub fn usage(args: &[String]) {
    println!(
        "basedtyper

        \rusage:\n \
        \r {arg} <path to wordlist> <count>        | fetches words and their definitions from a wordlist in a random order
        \r {arg} quote                             | fetches a random post from r/copypasta (UNSTABLE)
        
        \roptions:\n \
        \r --no-defs                       disable definitions for words
        ",
        arg = &args[0]
    );

    std::process::exit(0);
}
