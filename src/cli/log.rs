extern crate ansi_term;

use ansi_term::Color::Red;

pub fn err(s: String) {
    eprintln!("{} {}", Red.paint("ERR!"), s);
}

pub fn success(s: String) {
    println!("Successfully {}", s);
}