use clap::Parser;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value = "errors_outcome.log")]
    error_file: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.file)?;
    let reader = BufReader::new(file);

    let mut error_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&args.error_file)?;

    // Bug 1: think about borrow checker
    let mut line_count = 0;
    
    // Bug 1: think about borrow checker
    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;

        line_count += 1;

        if line.contains("error") {
            error_count += 1;
            writeln!(error_log, "Line {}: {}", line_count, line.to_lowercase())?;
        }
        // Bug 2: Only write to error log if line contains "error", lowercase it
        // Bug 3: Line count probably need to track current line

        // Task 1: Count words in line
        let word_count: usize = line.split_whitespace().count();
        println!("Line {}: {} words - {}", line_count, word_count, line);
    }

    println!("\nSummary:");
    // Task 2: Print total lines, error_count with errors, and error file path
    println!("Total lines: {}", line_count);
    println!("Errors: {}", error_count);
    println!("Error log: {}", args.error_file);

    Ok(())
}
