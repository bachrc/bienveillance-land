use crate::person::Person;
use crate::smoothtalker::{SmoothTalker, ComplimentRepository};

pub struct Welcomer {
    pub smooth_talker: SmoothTalker,
}

impl Welcomer {
    pub fn new(compliment_repository: Box<dyn ComplimentRepository>) -> Welcomer {
        Welcomer {
            smooth_talker: SmoothTalker::new(compliment_repository)
        }
    }

    pub fn compute_default_message(&self) -> String {
        String::from("Bonjour !")
    }

    pub fn compute_personnalized_message(&self, person_to_welcome: &Person) -> String {
        let welcome = format!("Bonjour {} !", person_to_welcome.compute_display_name());

        let compliment = self.smooth_talker.compliment_person(person_to_welcome);

        format!("{} {}", welcome, compliment)
    }
}
