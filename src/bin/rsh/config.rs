use std::env;

pub fn ps1() -> String {
    match env::var_os("PS1") {
        Some(value) => value.to_string_lossy().to_string(),
        None => format!("rsh-{} $ ", crate::version::VERSION_STR),
    }
}

pub fn ps2() -> String {
    match env::var_os("PS2") {
        Some(value) => value.to_string_lossy().to_string(),
        None => "> ".to_owned(),
    }
}

// pub struct Config {

// }

// impl Config {

// }
