mod timetable;

use chrono::naive::NaiveDate;
use chrono::Duration;
use std::io::stdin;
use std::str::FromStr;
use timetable::{Constraint, Timetable};

/// Prompt for user input. The prompt will repeat until the input is valid.
fn prompt<T: FromStr>(msg: &str) -> T {
    loop {
        println!("{}", msg);
        let mut buffer = String::new();
        if stdin().read_line(&mut buffer).is_err() {
            continue;
        }
        if let Ok(t) = buffer.trim().parse() {
            return t;
        }
    }
}

/// Prompt for the timetable constraint type.
fn prompt_constraint() -> Constraint {
    loop {
        let constraint_type: String = prompt("What constraint type do you want to use, arrival frequency or number of vehicles? (f/n)");
        match constraint_type.as_ref() {
            "f" | "freq" | "frequency" => {
                let frequency = prompt("How many days should be between each vehicle?");
                return Constraint::Frequency(frequency);
            }
            "n" | "nov" | "number of vehicles" => {
                let number_of_vehicles = prompt("How many vehicles should be on this route?");
                return Constraint::NumberOfVehicles(number_of_vehicles);
            }
            _ => println!("Constraint type not recognized."),
        }
    }
}

fn main() {
    loop {
        let start_date: NaiveDate =
            prompt("On which date should the timetable start? (yyyy-mm-dd)");
        let length = Duration::days(prompt("How many days should the timetable take?"));
        let constraint_type = prompt_constraint();

        let timetable = Timetable::from_constraint(start_date, length, constraint_type);
        for (i, val) in timetable.start_dates().enumerate() {
            println!("Vehicle {} starts on {}.", i + 1, val);
        }
        println!();
    }
}
