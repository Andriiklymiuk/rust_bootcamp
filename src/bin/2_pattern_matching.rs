fn main() {
    let status = match_network_status(404);
    let cmd = parse_command("GET /api/users");
    println!("Status: {:?}, Command: {:?}", status, cmd);
}

#[derive(Debug)]
enum NetworkStatus {
    Connected,
    Disconnected,
    Error(u16),
}

fn match_network_status(code: u16) -> NetworkStatus {
    match code {
        200 => NetworkStatus::Connected,
        404 => NetworkStatus::Error(404),
        // Bug 1: Missing catch-all pattern
        _ => NetworkStatus::Disconnected,
    }
}

fn parse_command(input: &str) -> (&str, &str) {
    // Bug 2: you need to somehow create array of strings from input
    let parts: Vec<&str> = input.split_whitespace().collect();

    (parts[0], parts[1])
}
