extern crate ansi_term;

use ansi_term::Color::Purple;

use std::collections::hash_map::HashMap;
use std::fs::write;
use std::path::PathBuf;

use super::class::Class;
use super::super::util::indent::indent_endl;
use super::super::cli::{log::*, input::Args};

pub enum SortingMethod {
    ID,
    Name,
    Period
}

#[derive(Debug)]
pub struct Classes {
    pub map: HashMap<String, Class>
}

impl Classes {
    pub fn new() -> Self {
        Classes {
            map: HashMap::new()
        }
    }

    pub fn add_class(&mut self, id: String, class: Class) {
        self.map.insert(id, class);
    }
    
    pub fn add_class_data(&mut self, id: String, name: String, period: usize) {
        let class = Class::new(id, name, period);
        self.add_class(class.id.clone(), class);
    }

    pub fn get_class(&mut self, args: &Args) -> Result<&mut Class, String> {
        match args.list.get(0) {
            Some(id) => {
                if !self.map.contains_key(id) {
                    Err(format!("Class '{}' not found", id))
                } else {
                    Ok(self.map.get_mut(id).unwrap())
                }
            }
            None => Err(format!("No ID provided {}", args.list()))
        }
    }

    pub fn remove_class(&mut self, id: &String) -> Result<Class, String> {
        if !self.map.contains_key(id) {
            Err(format!("Class '{}' not found", id))
        } else {
            Ok(self.map.remove(id).unwrap())
        }
    }

    pub fn sorted(&self, sort: SortingMethod) -> Vec<Class> {
        use SortingMethod::*;

        let mut values = self.map.values().cloned().collect::<Vec<Class>>();

        values.sort_by_key(|c| {
            match sort {
                ID => c.id.clone(),
                Name => c.name.clone(),
                Period => c.period.to_string()
            }
        });

        values
    }

    pub fn attach_class(s: &String, c: &String) -> String {
        format!("{} {}", s, Purple.bold().paint(format!("({})", c)))
    }

    pub fn attach_class_tag(s: &String, c: &String) -> String {
        format!("{} #{}", s, c)
    }

    pub fn attach_class_items(v: Vec<String>, c: &Class) -> Vec<String> {
        v.iter()
            .map(|s| Self::attach_class(s, &c.id.clone()))
            .collect::<Vec<String>>()
    }

    pub fn late(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for c in self.sorted(SortingMethod::Period) {
            result.append(&mut Self::attach_class_items(Class::completed_list(&c.late()), &c));
        }

        result
    }

    pub fn all_assignments(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for c in self.sorted(SortingMethod::Period) {
            result.append(&mut Self::attach_class_items(Class::assignment_list(&c.assignments, true), &c));
        }

        result
    }
    
    pub fn assignments_by_date(&self) -> HashMap<String, Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();

        for c in self.sorted(SortingMethod::Period) {
            for a in c.assignments {
                let date = a.due_date.format("%Y-%m-%d").to_string();
                let value = Self::attach_class_tag(&a.name, &c.id.clone());

                if result.contains_key(&date) {
                    result.get_mut(&date).unwrap().push(value);
                } else {
                    result.insert(date, vec![value]);
                }
            }
        }

        result
    }

    pub fn all_completed(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for c in self.sorted(SortingMethod::Period) {
            result.append(&mut Self::attach_class_items(Class::completed_list(&c.completed), &c));
        }

        result
    }

    pub fn display_all_info(&self) -> String {
        self.sorted(SortingMethod::Period)
            .iter()
            .map(|c| c.display_info_properties())
            .collect::<Vec<String>>()
            .join("\n\n")
    }

    pub fn display_all_combined(&self) -> String {
        let assignments = indent_endl(self.all_assignments(), 2);
        let completed = indent_endl(self.all_completed(), 2);

        Class::display_all_fmt(assignments, completed)
    }

    pub fn display_list(&self, sort: SortingMethod) -> String {
        self.sorted(sort).iter().map(|c| c.display()).collect::<Vec<String>>().join("\n")
    }

    pub fn klog(&self, avg: usize) -> String {
        let map = self.assignments_by_date();
        let mut result: Vec<String> = Vec::new();

        let t = format!("{}h", avg);

        for d in map.keys() {
            let vals = map.get(d).unwrap()
                .clone()
                .iter()
                .map(|v| format!("{} {}", t, v))
                .collect();

            let body = indent_endl(vals, 2);
            let s = format!("{}\n{}", d, body);

            result.push(s);
        }

        result.join("\n\n")
    }

    pub fn encode(&self) -> String {
        self.sorted(SortingMethod::Period).iter().map(|c| c.encode()).collect::<Vec<String>>().join("\n")
    }

    pub fn parse(s: String) -> Self {
        let lines = s.split("\n").filter(|l| !l.is_empty());

        let classes: Vec<Class> = lines.map(|l| Class::parse(l)).collect();
        let mut map = HashMap::<String, Class>::new();

        for c in classes {
            map.insert(c.id.clone(), c);
        }

        Classes {
            map
        }
    }

    pub fn write(&self, path: PathBuf, data: String) {
        match write(path.clone(), data) {
            Ok(_) => success(format!("wrote to '{}'", path.to_str().unwrap())),
            Err(e) => err(e.to_string())
        }
    }
}