extern crate ansi_term;
extern crate rustyline;

use std::collections::HashMap;
use std::path::PathBuf;

use ansi_term::Color::{Green};
use rustyline::Editor;

use super::{input::input, log::*};
use super::super::structs::{classes::{Classes, SortingMethod}, class::Class, assignment::Assignment};
use super::super::util::config::write_config;

fn to_int(arg: &String) -> Option<usize> {
    match arg.parse::<usize>() {
        Ok(n) => Some(n),
        Err(e) => {
            err(e.to_string());
            None
        }
    }
}

pub fn handler(classes: &mut Classes, rl: &mut Editor<()>) -> bool {
    let mut will_write = true;
    let mut help = HashMap::<&str, (Option<&str>, &str, Option<&str>)>::new();

    println!();

    let late = classes.late();

    if late.len() > 0 {
        println!("You have some late assignments!\n\n{}\n", late.join("\n"));
    }

    help.insert("add", (Some("<class> <date> <name...>"), "Adds a dated assignment to a class.\n\nDates should be formatted as 'd-m-y'.\nExample: 31-1-2021", Some("a")));
    help.insert("all", (None, "Displays assignments across all classes.", None));
    help.insert("clean", (Some("<class>"), "Removes all completed assignments from a class.", None));
    help.insert("complete", (Some("<class> <index>"), "Moves an assignment to a class's completed list.", Some("c")));
    help.insert("create", (Some("<id> <period> <name...>"), "Creates a class with metadata.", None));
    help.insert("delete", (Some("<id>"), "Deletes a class, including all of its assignments.", None));
    help.insert("encode", (None, "Displays encoded class data.", None));
    help.insert("help", (Some("(<command>)"), "Displays help info for a command.\nIf no command is supplied, displays all commands.", Some("h")));
    help.insert("info", (Some("(<id>)"), "Displays class info and assignments.\nIf no ID is supplied, displays all class info.", Some("i")));
    help.insert("klog", (Some("<avg> (<path>)"), "Displays assignment data in klog format.\nThis is particularly useful for keeping track of\nassignments you've completed with date and time.\n\nThe 'avg' argument is how many hours on average\nyou'd expect to complete the assignments in.\nYou can modify these values after writing.\n\nOptionally specify a path to write to.\n'.klg' is automatically appended to the path.\n\nLearn more about klog at: https://klog.jotaen.net", None));
    help.insert("list", (Some("(<sort>)"), "Lists all classes by ID and name.\nYou can sort classes by id, name and period (default).", Some("ls, l")));
    help.insert("modify", (Some("<id> <property> <value...>"), "Modifies class metadata by input.\nClass ID cannot be modified.", Some("mod, m")));
    help.insert("panic", (None, "Prevents writing to config upon exiting the program.\nThis is useful if you've made an irreversible mistake while editing.", None));
    help.insert("quit", (None, "Exits the program.", Some("q")));
    help.insert("remove", (Some("<id> <index>"), "Removes an assignment without completing it.", Some("r")));
    help.insert("write", (None, "Writes encoded classes to the config file.\nThis is done automatically upon exit.", Some("w")));

    loop {
        match input(rl) {
            Some(args) => {
                //println!("{:#?}", args);
                match args.command.as_str() {
                    "add" | "a" => {
                        if args.check(3, true) {
                            match classes.get_class(&args) {
                                Ok(c) => {
                                    match Assignment::parse_date(args.list[1].as_str()) {
                                        Ok(d) => {
                                            c.add_assignment(args.input_from(2), d);
                                            println!("\n{}", c.display_info());
                                        }
                                        Err(e) => err(e)
                                    }
                                },
                                Err(e) => err(e)
                            }  
                        }
                    }
                    "all" => {
                        println!("\n{}", classes.display_all_combined());
                    }
                    "clean" => {
                        if args.check(1, true) {
                            match classes.get_class(&args) {
                                Ok(c) => {
                                    c.completed.clear();
                                    println!("\n{}", c.display_info());
                                },
                                Err(e) => err(e)
                            }
                        }
                    }
                    "complete" | "c" => {
                        if args.check(2, true) {
                            match classes.get_class(&args) {
                                Ok(c) => {
                                    match to_int(&args.list[1]) {
                                        Some(n) => {
                                            match c.complete_assignment(n) {
                                                Ok(_) => println!("\n{}", c.display_info()),
                                                Err(e) => err(e)
                                            }
                                        }
                                        None => ()
                                    }
                                },
                                Err(e) => err(e)
                            }  
                        }
                    }
                    "create" => {
                        if args.check(3, true) {
                            let id = args.list[0].to_lowercase();

                            if classes.map.contains_key(&id) {
                                err(format!("Class ID '{}' already exists", id));
                            } else {
                                match to_int(&args.list[1]) {
                                    Some(n) => {
                                        classes.add_class_data(id.clone(), args.input_from(2), n);
                                        success(format!("created class '{}'", id));
                                    }
                                    None => ()
                                }
                            }  
                        }
                    }
                    "delete" => {
                        if args.check(1, true) {
                            match classes.remove_class(&args.list[0]) {
                                Ok(c) => success(format!("deleted class '{}'", c.name)),
                                Err(e) => err(e)
                            }
                        }
                    }
                    "encode" => {
                        println!("\n{}", classes.encode());
                    }
                    "help" | "h" => {
                        if args.check(1, false) {
                            if help.contains_key(&args.list[0].as_str()) {
                                let data = help.get(&args.list[0].as_str()).unwrap();

                                let arg_str = match data.0 {
                                    Some(s) => String::from(s),
                                    None => String::new()
                                };

                                println!("\n{}\n\n{}{}",
                                    Class::str_property("Syntax", &format!("{} {}", args.list[0], arg_str)),
                                    
                                    data.1,

                                    if data.2.is_some() { 
                                        format!("\n\n{}", Class::str_property("Aliases", &data.2.unwrap().to_string())) 
                                    } else { 
                                        String::new()
                                    }
                                );
                            } else {
                                err(format!("Command '{}' not found", args.list[0]));
                            }
                        } else {
                            let mut sorted_keys = help.keys().collect::<Vec<&&str>>();

                            sorted_keys.sort();
                            
                            let lines = sorted_keys
                                .iter()
                                .map(|k| {
                                    let l = help.get(*k).unwrap().1.split("\n").collect::<Vec<&str>>()[0];
                                    Class::info_property(k, &l.to_string(), Green.bold())
                                })
                                .collect::<Vec<String>>();
                                
                            println!("\n{}", lines.join("\n"));
                        }
                    }
                    "info" | "i" => {
                        if args.check(1, false) {
                            match classes.get_class(&args) {
                                Ok(c) => println!("\n{}", c.display_info()),
                                Err(e) => err(e)
                            }
                        } else {
                            println!("\n{}", classes.display_all_info());
                        }
                    }
                    "klog" => {
                        if args.check(1, true) {
                            match to_int(&args.list[0]) {
                                Some(n) => {
                                    let data = classes.klog(n);
                                    
                                    if args.check(2, false) {
                                        let mut path = PathBuf::from(args.list[1].clone());
                                        path.set_extension("klg");

                                        classes.write(path, data);
                                    } else {
                                        println!("\n{}", data);
                                    }
                                }
                                None => ()
                            }
                        }
                    }
                    "list" | "ls" | "l" => {
                        use SortingMethod::*;

                        let by_period: Option<SortingMethod> = if args.check(1, false) {
                            match args.list[0].to_lowercase().as_str() {
                                "id" => Some(ID),
                                "name" => Some(Name),
                                "period" => Some(Period),
                                _ => None
                            }
                        } else { 
                            Some(Period)
                        };

                        match by_period {
                            Some(v) => println!("\n{}", classes.display_list(v)),
                            None => err(format!("Invalid sorting method '{}'", args.list[0]))
                        }
                    }
                    "modify" | "mod" | "m" => {
                        if args.check(3, true) {
                            match classes.get_class(&args) {
                                Ok(c) => {
                                    match c.modify(args.list[1].clone(), args.input_from(2)) {
                                        Ok(_) => success(format!("modified '{}'", c.name)),
                                        Err(e) => err(e)
                                    }
                                },
                                Err(e) => err(e)
                            }
                        }
                    }
                    "panic" => {
                        will_write = false;
                        success(String::from("prevented write on shutdown. None of the changes made during this session will be saved.\nTo view the encoded version of the changes you've made, run 'encode'."));
                    }
                    "quit" | "q" => {
                        print!("Exiting... ");
                        break;
                    }
                    "remove" | "r" => {
                        if args.check(2, true) {
                            match classes.get_class(&args) {
                                Ok(c) => {
                                    match to_int(&args.list[1]) {
                                        Some(n) => {
                                            match c.remove_assignment(n) {
                                                Ok(_) => println!("\n{}", c.display_info()),
                                                Err(e) => err(e)
                                            }
                                        }
                                        None => ()
                                    }
                                },
                                Err(e) => err(e)
                            }  
                        }
                    }
                    "write" | "w" => {
                        write_config(&classes);
                    }
                    _ => err(format!("Unrecognized command '{}'. Run 'help' for a list of commands.", args.command))
                }

                println!();
            }
            None => { 
                print!("Exiting... ");
                break;
            }
        }
    }

    will_write
}