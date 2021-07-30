use crate::person::Person;

pub struct Welcomer;

impl Welcomer {
    pub fn compute_welcome_message() -> String {
        String::from("Bonjour !")
    }

    pub fn compute_personnalized_welcome_message(person_to_welcome: &Person) -> String {
        String::from(
            format!("Bonjour {} ! Vous êtes pimpant.e aujourd'hui !", person_to_welcome.compute_display_name())
        )
    }
}
