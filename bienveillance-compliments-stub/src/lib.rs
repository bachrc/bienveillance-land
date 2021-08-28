use bienveillance_core::smoothtalker::{Sentence, ComplimentRepository};
use bienveillance_core::smoothtalker::SentenceComponent::{Invariant, Variant};
use rand::seq::SliceRandom;

pub struct ComplimentsStub{}

impl ComplimentRepository for ComplimentsStub {
    fn fetch_compliment(&self) -> Sentence {
        self.available_compliments()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}

impl ComplimentsStub {
    fn available_compliments(&self) -> Vec<Sentence> {
        vec![
            Sentence {components : vec![
                Invariant(String::from("Tu es très bien")),
                Variant {
                    inclusive_form: String::from("fringué.e"),
                    masculine_form: String::from("fringué"),
                    feminine_form: String::from("fringuée")
                },
                Invariant(String::from("aujourd'hui !")),
                Variant {
                    inclusive_form: String::from("BG"),
                    masculine_form: String::from("Beau gosse"),
                    feminine_form: String::from("Belle gosse")
                },
                Invariant(String::from("va !"))
            ]},
            Sentence { components : vec![
                Invariant(String::from("Tu es")),
                Variant {
                    inclusive_form: String::from("fabuleu.x.se"),
                    masculine_form: String::from("fabuleux"),
                    feminine_form: String::from("fabuleuse")
                },
                Invariant(String::from("et")),
                Variant {
                    inclusive_form: String::from("fort.e"),
                    masculine_form: String::from("fort"),
                    feminine_form: String::from("forte")
                },
                Invariant(String::from(", bombe le torse car tu peux être")),
                Variant {
                    inclusive_form: String::from("fier.e"),
                    masculine_form: String::from("fier"),
                    feminine_form: String::from("fière")
                },
                Invariant(String::from("!"))
            ]}
        ]
    }
}
