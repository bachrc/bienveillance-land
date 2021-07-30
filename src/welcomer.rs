use crate::person::Person;

pub struct Welcomer;

impl Welcomer {
    pub fn compute_default_welcome_message(&self) -> String {
        String::from("Bonjour !")
    }

    pub fn compute_personnalized_welcome_message(&self, person_to_welcome: &Person) -> String {
        let welcome = format!("Bonjour {} !", person_to_welcome.compute_display_name());

        let compliment = String::from(match person_to_welcome.pronoun {
            None => "Vous êtes pimpant.e aujourd'hui !",
            Some(_) => "Vous êtes pimpante aujourd'hui !"
        });

        format!("{} {}", welcome, compliment)
    }
}
