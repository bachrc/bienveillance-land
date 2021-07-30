pub enum Pronoun {
    Feminine
}

pub struct Person {
    first_name: String,
    last_name: String,
    pub(crate) pronoun: Option<Pronoun>,
}

impl Person {
    pub fn with_name(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            pronoun: None,
        }
    }

    pub fn with_name_and_pronoun(first_name: &str, last_name: &str, pronoun: Pronoun) -> Person {
        Person {
            pronoun: Some(pronoun),
            ..Person::with_name(first_name, last_name)
        }
    }

    pub fn compute_display_name(&self) -> String {
        String::from(format!("{} {}", self.first_name, self.last_name))
    }
}
