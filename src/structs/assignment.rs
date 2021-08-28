extern crate ansi_term;
extern crate chrono;

use ansi_term::Color::Green;
use chrono::{offset::Local, Datelike, NaiveDate};

use super::super::cli::log::err;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: String,
    pub due_date: NaiveDate
}

impl Assignment {
    pub fn new(name: String, due_date: NaiveDate) -> Self {
        Assignment {
            name,
            due_date,
        }
    }

    pub fn display(&self, spaces: usize) -> String {
        format!("{}{}- {}", 
            Green.bold().paint(&self.name), 
            " ".repeat(spaces + 1), 
            Green.paint(self.due_date.format("%b %e %Y").to_string())
        )
    }

    pub fn encode(&self) -> String {
        format!("[{};{:0>2}-{:0>2}-{:4}]", self.name, self.due_date.day(), self.due_date.month(), self.due_date.year())
    }

    pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
        match NaiveDate::parse_from_str(s, "%d-%m-%Y") {
            Ok(d) => Ok(d),
            Err(e) => Err(format!("Failed to parse date '{}': {}", s, e))
        }
    }

    pub fn parse(s: &str) -> Self {
        let args: Vec<&str> = s[1..s.len() - 1].split(";").collect();
        let due_date = match Self::parse_date(args[1]) {
            Ok(d) => d,
            Err(e) => {
                err(e);
                Local::today().naive_local()
            }
        };

        Assignment {
            name: String::from(args[0]),
            due_date
        }
    }
}