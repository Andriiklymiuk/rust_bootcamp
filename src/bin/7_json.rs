use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::{self, BufReader},
};

#[derive(Debug, Deserialize)]
struct User {
    nickname: String,
    role: String,
}

fn read_users(path: &str) -> io::Result<Vec<User>> {
    let file = File::open(path)?;
    serde_json::from_reader(BufReader::new(file))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <users.json>", args[0]);
        // Bug 1: Exit with error code 1
        std::process;
    }

    // Bug 2: Something is missing here, check the type
    let users = read_users(&args[1]);

    // Bug 3: Are we sure, that we get users count here?
    let admin_count = users.iter().filter(|u| u.role == "admin");

    println!("Found {} admins:", admin_count);

    // Bug 4: Something is missing here, check the type
    users
        .filter(|u| u.role == "admin")
        .for_each(|u| println!("- {}", u.nickname));

    Ok(())
}
