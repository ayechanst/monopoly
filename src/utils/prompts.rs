use std::io::{self, Write};

pub fn prompt_player(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // makes the prompt display immidiately
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
