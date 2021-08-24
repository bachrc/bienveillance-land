use crate::person::{Person, Pronoun};
use crate::smoothtalker::SentenceComponent::{Invariant, Variant};

pub struct Sentence {
    pub components: Vec<SentenceComponent>,
}

impl Sentence {
    pub fn display_sentence_for_pronouns(&self, pronouns: &Option<Pronoun>) -> String {
        self.components.iter()
            .map(|component| component.compute_sentence_for_pronouns(pronouns))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

pub enum SentenceComponent {
    Invariant(String),
    Variant {
        inclusive_form: String,
        masculine_form: String,
        feminine_form: String,
    },
}

impl SentenceComponent {
    pub fn compute_sentence_for_pronouns(&self, pronouns: &Option<Pronoun>) -> String {
        String::from(match self {
            SentenceComponent::Invariant(words) => words,
            SentenceComponent::Variant { inclusive_form: inclusive, feminine_form, masculine_form } => match pronouns {
                None => inclusive,
                Some(pronoun) => match pronoun {
                    Pronoun::Feminine => feminine_form,
                    Pronoun::Masculine => masculine_form
                }
            }
        })
    }
}

pub struct SmoothTalker;

impl SmoothTalker {
    pub fn compliment_person(&self, person: &Person) -> String {
        let default_compliment = Sentence {
            components: vec![
                Invariant(String::from("Vous êtes")),
                Variant {
                    inclusive_form: String::from("pimpant·e"),
                    masculine_form: String::from("pimpant"),
                    feminine_form: String::from("pimpante"),
                },
                Invariant(String::from("aujourd'hui !"))
            ]
        };

        default_compliment.display_sentence_for_pronouns(&person.pronoun)
    }
}