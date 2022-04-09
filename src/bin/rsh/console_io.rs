use std::io::{self, Write};

use crate::config;

pub fn get_input() -> String {
    let mut input_buffer = String::new();
    print!("{}", config::ps1());
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Fatal error reading from STDIN");

    String::from(input_buffer.trim())
}
