use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;

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
        println!("{}", line);
    }

    Ok(())
}