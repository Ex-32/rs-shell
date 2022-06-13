use std::io::{self, Write};

use crate::config;

pub fn get_command() -> String {
    print!("{}", config::ps1());
    get_input()
}

pub fn get_followup() -> String {
    print!("{}", config::ps2());
    get_input()
}

fn get_input() -> String {
    let mut input_buffer = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input_buffer)
        .expect("fatal error reading from stdin");
    String::from(input_buffer)
}
