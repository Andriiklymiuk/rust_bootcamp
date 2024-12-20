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
        // Task 1: print line, if it contains error
        // Task 2: separate the line into words and print time too
        match line {
            Ok(result) => {
                if result.contains("error") {
                    println!("Line: {}", result)
                } else {
                    let words: Vec<&str> = result.split_whitespace().take(2).collect();
                    println!("No error: {} {}", words[0], words[1])
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }

    Ok(())
}
