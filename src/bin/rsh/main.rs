mod console_io;
mod config;
mod parser;
mod ast;
mod interpreter;
mod version;

fn main() {

    let trailing_backslash_re = regex::Regex::new(r"\\\s*$")
        .expect("fatal error compiling regex");
    let empty_re = regex::Regex::new(r"^\s*$")
        .expect("fatal error compiling regex");

    loop {
        let mut buffer = console_io::get_command();
        if empty_re.is_match(&buffer) {
            continue;
        }

        while trailing_backslash_re.is_match(&buffer) {
            buffer.push_str(&console_io::get_followup());
        }

       let _ = parser::parse_command(&buffer);

        print!("{}",buffer)
    }
}
