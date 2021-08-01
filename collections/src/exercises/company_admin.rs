use std::{collections::HashMap, io};
use regex::{Regex, RegexSet};

use crate::exercises::company_admin::utils::print_help;

pub fn handle_input() {
  use types::Name;

  let mut employees: types::Employees = HashMap::new();
  
  let commands = RegexSet::new(&[
    r"add (?P<name>\w+) to (?P<department>\w+)",
    r"view all",
    r"view (?P<department>\w+)",
    r"help",
    r"exit",
    r"quit",
    ]).unwrap();
    
  loop {
    let mut input = String::new();
    if let Err(error) = io::stdin().read_line(&mut input) {
      return println!("Unable to read input, error = {}", error);
    }
    input = input.to_lowercase();

    let matches = commands.matches(&input);
    let matched = matches.iter().next();

    match matched {
      Some(idx) => {
        let re = Regex::new(&commands.patterns()[idx]).unwrap();
        let caps = re.captures(&input).unwrap();

        match idx {
          0 => {
            let dept = utils::string_to_department(&caps["department"]);

            if let Some(d) = dept {
              create::add_employee(&Name(caps["name"].to_string()), &d, &mut employees);
              println!("Added employee successfully!")
            } else {
              println!("I couldn't find that department")
            }
          }
          1 => read::view_all_employees(&employees),
          2 => {
            let dept = utils::string_to_department(&caps["department"]);

            if let Some(d) = dept {
              read::view_department_employees(&d, &employees);
            } else {
              println!("I couldn't find that department")
            }
          },
          3 => print_help(commands.patterns()),
          4 | 5 => return,
          _ => (),
        }
      },
      None => println!("Sorry, I didn't understand that. Please try again!"),
    }
  }
}

mod types {
  use std::collections::HashMap;

  #[derive(Hash, PartialEq, Eq, Clone, Copy)]
  pub enum Department {
    Engineering,
    Sales,
  }
  
  #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
  pub struct Name(pub String);

  pub type Employees = HashMap<Department, Vec<Name>>;
}

mod read {
  use crate::exercises::company_admin::utils;

use super::types::{Department, Employees};

  pub fn view_department_employees(dept: &Department, employees: &Employees) {
    let dept_employees = employees.get(dept).unwrap();
    println!("{:?}", utils::sort_names(dept_employees));
  }

  pub fn view_all_employees(employees: &Employees) {
    let mut all_employees = Vec::new();

    for (_, names) in employees {
      names.iter().for_each(|n| all_employees.push(n.clone()));
    }

    println!("{:?}", utils::sort_names(&all_employees));
  }
}

mod create {
  use super::types::{Name, Department, Employees};

  pub fn add_employee(name: &Name, dept: &Department, employees: &mut Employees) {
    employees
      .entry(*dept)
      .and_modify(|es| es.push(name.clone()))
      .or_insert(vec![name.clone()]);
  }
}

mod utils {
  use super::types::{Department, Name};

  pub fn string_to_department(input: &str) -> Option<Department> {
    println!("input = {}", input);
    match input {
      "engineering" => Some(Department::Engineering),
      "sales" => Some(Department::Sales),
      _ => None,
    }
  }

  pub fn print_help(commands: &[String]) {
    println!("Available commands:\n{:#?}", commands);
  }

  pub fn sort_names(names: &Vec<Name>) -> Vec<Name> {
    let mut sorted = names.clone();
    sorted.sort();
    sorted
  }
}