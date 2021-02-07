use std::io;

pub enum Action {
    List(),
    Search(String),
    Invalid(),
}

pub fn get_action(input: String) -> Action {
    return match input.to_lowercase().trim() {
        "l" | "ls" | "list" => Action::List(),
        "s" | "search" => {
            println!("What would you like to search for?");

            let mut search_term = String::new();

            io::stdin()
                .read_line(&mut search_term)
                .expect("Failed to read line");

            Action::Search(search_term)
        }
        _ => Action::Invalid(),
    };
}
