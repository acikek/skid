extern crate ansi_term;
extern crate chrono;

use std::collections::LinkedList;

use ansi_term::{Style, Color::{Blue, Green, Yellow, Black}};
use chrono::{NaiveDate, Local, Datelike};

use super::assignment::Assignment;
use super::super::util::indent::indent_endl;

#[derive(Debug, Clone)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub period: usize,
    pub assignments: Vec<Assignment>,
    pub completed: Vec<String>
}

impl Class {
    pub fn new(id: String, name: String, period: usize) -> Self {
        Class {
            id,
            name,
            period,
            assignments: Vec::new(),
            completed: Vec::new()
        }
    }

    pub fn add_assignment(&mut self, name: String, due_date: NaiveDate) {
        self.assignments.push(Assignment::new(name, due_date));
    }

    pub fn remove_assignment(&mut self, index: usize) -> Result<String, String> {
        if self.assignments.len() >= index {
            let removed = self.assignments.remove(index - 1);
            Ok(removed.name)
        } else {
            Err(format!("No assignment at index {}", index))
        }   
    }

    pub fn complete_assignment(&mut self, index: usize) -> Result<(), String> {
        match self.remove_assignment(index) {
            Ok(name) => { self.completed.push(name); Ok(()) }
            Err(e) => Err(e)
        }
    }

    pub fn modify(&mut self, property: String, value: String) -> Result<(), String> {
        match property.to_lowercase().as_str() {
            "name" => { self.name = value; Ok(()) },
            "period" => {
                match value.parse::<usize>() {
                    Ok(n) => { self.period = n; Ok(()) },
                    Err(e) => Err(format!("Invalid period '{}': {}", value, e))
                }
            },
            _ => Err(format!("Invalid property '{}'", property))
        }
    }

    pub fn late(&self) -> Vec<String> {
        let today = Local::today().naive_local().num_days_from_ce();

        self.assignments.iter()
            .filter(|a| a.due_date.num_days_from_ce() < today)
            .map(|a| a.name.clone())
            .collect::<Vec<String>>()
    }

    pub fn assignment_list(assignments: &Vec<Assignment>) -> Vec<String> {
        let mut len: usize = 0;

        for s in assignments {
            if s.name.len() > len {
                len = s.name.len();
            }
        }

        assignments
            .iter()
            .enumerate()
            .map(|(i, a)| format!("{}{} {}", 
                Yellow.bold().paint((i + 1).to_string()), 
                Black.bold().paint(")"),
                a.display(len - a.name.len())))
            .collect()
    }

    pub fn completed_list(completed: &Vec<String>) -> Vec<String> {
        completed
            .clone()
            .into_iter()
            .map(|c| format!("{} {}", 
                Black.bold().paint("-"),
                Green.bold().paint(c)))
            .collect()
    }

    pub fn list_none(s: String) -> String {
        if !s.is_empty() {
            format!("\n{}", s)
        } else {
            String::from("None")
        }
    }

    pub fn info_property(property: &str, value: &String, color: Style) -> String {
        format!("{}: {}", Blue.paint(property), color.paint(value))
    }

    pub fn str_property(property: &str, value: &String) -> String {
        Self::info_property(property, value, Green.bold())
    }

    pub fn int_property(property: &str, value: &String) -> String {
        Self::info_property(property, value, Yellow.bold())
    }

    pub fn display(&self) -> String {
        Self::str_property(&self.id, &self.name)
    }

    pub fn display_all_fmt(assignments: String, completed: String) -> String {
        format!("{}: {}\n\n{}: {}", 
            Blue.paint("Assignments"),
            Self::list_none(assignments), 
            Blue.paint("Completed"),
            Self::list_none(completed)
        )
    }

    pub fn display_all(&self) -> String {
        let assignments = indent_endl(Self::assignment_list(&self.assignments), 2);
        let completed = indent_endl(Self::completed_list(&self.completed), 2);
        
        Self::display_all_fmt(assignments, completed)
    }

    pub fn display_info_properties(&self) -> String {
        vec![
            Self::str_property("Name", &self.name),
            Self::str_property("ID", &self.id),
            Self::int_property("Period", &self.period.to_string())
        ].join("\n")
    }

    pub fn display_info(&self) -> String {
        format!("{}\n\n{}",
            self.display_info_properties(),
            self.display_all()
        )
    }

    pub fn encode(&self) -> String {
        let assignments: Vec<String> = self.assignments.iter().map(|a| a.encode()).collect();
        let mut args: Vec<String> = vec![self.id.clone(), self.name.clone(), self.period.to_string()];

        if !assignments.is_empty() { args.push(assignments.join(",")) }
        if !self.completed.is_empty() { args.push(self.completed.join(",")) }

        args.join(",")
    }

    pub fn parse(s: &str) -> Self {
        let mut args: LinkedList<&str> = s.split(",").collect();

        let id = String::from(args.pop_front().unwrap());
        let name = String::from(args.pop_front().unwrap());
        let period = args.pop_front().unwrap().parse::<usize>().unwrap();

        let mut assignments: Vec<Assignment> = vec![];
        let mut completed: Vec<String> = vec![];

        for a in args {
            if a.trim().starts_with("[") {
                assignments.push(Assignment::parse(a));
            } else {
                completed.push(String::from(a));
            }
        }

        Class {
            id,
            name,
            period,
            assignments,
            completed
        }
    }
}