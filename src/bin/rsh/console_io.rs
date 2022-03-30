use std::io::{self,Write};

use crate::config;

pub fn get_input() -> String {
    // prompt the user for input
    let mut input = String::new();
    print!("{}", config::ps1());
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Fatal error reading from STDIN");

    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }

    input
}
