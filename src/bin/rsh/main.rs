mod console_io;
mod config;
mod parser;
mod ast;
mod version;

fn main() {

    loop {
        let buffer = console_io::get_input();
        println!("{}",buffer)
    }

}
