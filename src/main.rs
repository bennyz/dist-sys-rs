use dist_sys_rs::{Message, Node};
use std::{
    fs::OpenOptions,
    io::{self, BufRead, Write},
    time::{SystemTime, UNIX_EPOCH},
};

fn log_to_file(message: &str, is_response: bool) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/log.txt")?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let log_entry = format!(
        "{} - {}: {}\n",
        timestamp,
        if is_response { "Response" } else { "Request" },
        message
    );

    file.write_all(log_entry.as_bytes())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut node = Node::new();

    for line in stdin.lock().lines() {
        let input = line?;
        log_to_file(&input, false)?;

        let input = serde_json::from_str::<Message>(&input)?;
        if let Some(response) = node.handle(input) {
            let response = serde_json::to_string(&response)?;
            log_to_file(&response, true)?;
            writeln!(stdout, "{}", response)?;
            stdout.flush()?;
        }
    }

    Ok(())
}
