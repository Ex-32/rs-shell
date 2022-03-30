use std::env;

use crate::version;

pub fn ps1() -> String {
    match env::var("PS1") {
        Ok(value) => value,
        Err(_) => String::from(format!("rsh-{} $ ", version::STR))
    }
}

pub fn ps2() -> String {
    match env::var("PS2") {
        Ok(value) => value,
        Err(_) => String::from("> ")
    }
}

// pub struct Config {

// }

// impl Config {




// }
