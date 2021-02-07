use std::fs;

pub struct Rule {
    pub number: i32,
    pub definition: String,
    pub applies_to_admins: bool,
    pub is_guideline: bool,
}

pub fn read_rules(file_name: &String) -> Vec<Rule> {
    let rules_file = fs::read_to_string(file_name).expect("Could not read file");
    let mut rules = Vec::new();

    for string in rules_file.split("\n") {
        if string.starts_with("#") || string.trim().is_empty() {
            continue;
        }

        let split: Vec<&str> = string.split(" : ").collect();

        rules.push(Rule {
            number: split[0].parse().unwrap(),
            definition: String::from(split[1]),
            applies_to_admins: split.get(2).unwrap_or_else(|| &"true").parse().unwrap(),
            is_guideline: split.get(3).unwrap_or_else(|| &"false").parse().unwrap(),
        })
    }

    return rules;
}

pub fn rule_to_string(rule: &Rule) -> String {
    return format!(
        "{} Number: {}\nDefinition: {}\nApplies to admins: {}",
        if rule.is_guideline {
            "Guideline"
        } else {
            "Rule"
        },
        rule.number,
        rule.definition,
        rule.applies_to_admins
    );
}
