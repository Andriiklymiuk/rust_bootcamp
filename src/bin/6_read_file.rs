use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        // Task 1: Count total number of words in the line

        println!("{}", line);
    }

    // Task 2: Count total number of lines in the file

    Ok(())
}
