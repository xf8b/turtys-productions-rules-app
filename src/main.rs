use std::{env, io};

use crate::actions::{get_action, Action};
use crate::rules::Rule;
use crate::rules::{read_rules, rule_to_string};

mod actions;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let rules = read_rules(file);

    println!("What would you like to do?");

    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");

    println!();

    match get_action(action) {
        Action::List() => {
            for rule in rules {
                println!("{}", rule_to_string(&rule));
                println!();
            }
        }
        Action::Search(search_term) => {
            println!();

            let result: Vec<&Rule> = rules
                .iter()
                .filter(|&rule| {
                    rule.definition
                        .to_lowercase()
                        .contains(search_term.to_lowercase().trim())
                })
                .collect();

            if !result.is_empty() {
                for rule in result {
                    println!("{}", rule_to_string(rule));
                    println!();
                }
            } else {
                panic!("error: could not find rule(s) matching {}", search_term)
            }
        }
        _ => panic!("error: invalid action"),
    }
}
