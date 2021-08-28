extern crate ansi_term;
extern crate rustyline;

use ansi_term::Color::Blue;
use rustyline::{Editor, error::ReadlineError};

use super::log::err;

#[derive(Debug)]
pub struct Args {
    pub command: String,
    pub list: Vec<String>,
    pub input: String
}

impl Args {
    pub fn list(&self) -> String {
        format!("(args: {})", self.list.join(", "))
    }

    pub fn check(&self, min: usize, strict: bool) -> bool {
        let result = self.list.len() >= min;
        
        if !result && strict { 
            let name = if min != 1 { "arguments" } else { "argument" };
            err(format!("Expected {} {} ({} provided)", min, name, self.list.len())) 
        }

        result
    }

    pub fn input_from(&self, offset: usize) -> String {
        self.list[offset..].join(" ")
    }
}

pub fn input(rl: &mut Editor<()>) -> Option<Args> {
    use ReadlineError::*;

    let prompt = format!("{} ", Blue.bold().paint("=>"));

    match rl.readline(prompt.as_str()) {
        Ok(l) => {
            let line = l.trim();
            let a: Vec<&str> = line.split(" ").collect();

            let command = String::from(a[0]);
            let list: Vec<String> = (&a[1..])
                .to_vec()
                .iter()
                .map(|s| String::from(*s))
                .filter(|s| !s.is_empty())
                .collect();

            let input = list.join(" ");

            rl.add_history_entry(line);

            Some(Args {
                command,
                list,
                input
            })
        }
        Err(Interrupted) | Err(Eof) => None,
        Err(e) => {
            err(e.to_string());
            None
        }
    }
}