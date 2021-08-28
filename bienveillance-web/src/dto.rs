use bienveillance_core::person::{Person};
type DomainPronoun = bienveillance_core::person::Pronoun;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PersonToWelcome {
    pub first_name: String,
    pub last_name: String,
    pub pronoun: Option<Pronoun>
}

impl From<PersonToWelcome> for Person {
    fn from(person: PersonToWelcome) -> Self {
        Self {
            first_name: person.first_name,
            last_name: person.last_name,
            pronoun: match person.pronoun {
                None => None,
                Some(pronoun) => Some(pronoun.into())
            }
        }
    }
}

#[derive(Deserialize)]
pub enum Pronoun {
    Feminine,
    Masculine,
}

impl From<Pronoun> for DomainPronoun {
    fn from(pronoun: Pronoun) -> Self {
        match pronoun {
            Pronoun::Feminine => DomainPronoun::Feminine,
            Pronoun::Masculine => DomainPronoun::Masculine
        }
    }
}
