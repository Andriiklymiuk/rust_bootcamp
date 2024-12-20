use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} errors.log", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        // Task 1: print line, if it contains error
        if line.contains("error") {
            println!("\x1b[31m{}\x1b[0m", line);
        }
        // Task 2: separate the line into words and print time too
        let mut words: Vec<&str> = line.split_whitespace().collect();
        let time = words[0];
        words = words[2..].to_vec();
        println!("Time: {}, Words: {:?}", time, words);
    }

    Ok(())
}
