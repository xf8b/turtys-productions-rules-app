pub struct Rule {
    pub number: i32,
    pub definition: String,
    pub applies_to_admins: bool,
    pub is_guideline: bool,
}

pub fn create_rule(number: i32, definition: &str) -> Rule {
    return Rule {
        number,
        definition: String::from(definition),
        applies_to_admins: true,
        is_guideline: false,
    };
}

pub fn create_non_admin_rule(number: i32, definition: &str) -> Rule {
    return Rule {
        number,
        definition: String::from(definition),
        applies_to_admins: false,
        is_guideline: false,
    };
}

pub fn create_guideline(number: i32, definition: &str) -> Rule {
    return Rule {
        number,
        definition: String::from(definition),
        applies_to_admins: true,
        is_guideline: true,
    };
}
