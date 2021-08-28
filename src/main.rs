mod cli;
mod structs;
mod util;

extern crate rustyline;

use rustyline::Editor;

use util::config::*;
use cli::handler::handler;

const HELP: &str = r#"
skid - class assignment scheduler
MIT (c) 2021 Kyle P.

Run 'help' for a list of commands.
Specify a command to view extra help
for that command.

Get started by creating some classes
and adding any assignments you have.

Run 'quit' or press Ctrl+C to exit."#;

fn main() { 
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();

    if !config_exists() {
        println!("{}", HELP);
        create_config();
    }

    let mut rl = Editor::<()>::new();
    let mut classes = read_config();

    let will_write = handler(&mut classes, &mut rl);

    if will_write {
        write_config(&classes);
    } else {
        println!();
    }
}
