use Pronoun::{Feminine, Masculine};

use crate::person::{Person, Pronoun};

pub struct Welcomer;

impl Welcomer {
    pub fn compute_default_message(&self) -> String {
        String::from("Bonjour !")
    }

    pub fn compute_personnalized_message(&self, person_to_welcome: &Person) -> String {
        let welcome = format!("Bonjour {} !", person_to_welcome.compute_display_name());

        let compliment = String::from(match &person_to_welcome.pronoun {
            None => "Vous êtes pimpant·e aujourd'hui !",
            Some(pronoun) => match pronoun {
                Feminine => "Vous êtes pimpante aujourd'hui !",
                Masculine => "Vous êtes pimpant aujourd'hui !"
            }
        });

        format!("{} {}", welcome, compliment)
    }
}
