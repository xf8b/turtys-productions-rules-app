use std::{env, io, process};

use crate::actions::{Action, get_action};
use crate::rules::{read_rules, rule_to_string};
use crate::rules::Rule;

mod actions;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).unwrap_or_else(|| {
        eprintln!("error: you need to pass in the file to read rules from");
        eprintln!("error: please see the GitHub for an example of a file");
        eprintln!("error: https://github.com/xf8b/turtys-productions-rules-app/blob/main/rules.txt");
        process::exit(1);
    });
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
                eprintln!("error: could not find any rule definitions containing {}", search_term);
                process::exit(3);
            }
        }
        Action::Invalid() => {
            eprintln!("error: invalid action");
            eprintln!("error: the available actions are: 'l'/'ls'/'list' and 's'/'search'");
            process::exit(2);
        }
    }
}
