use mockall::mock;

use bienveillance_core::person::Person;
use bienveillance_core::person::Pronoun;
use bienveillance_core::smoothtalker::{ComplimentRepository, Sentence, SmoothTalker};
use bienveillance_core::smoothtalker::SentenceComponent::{Invariant, Variant};
use bienveillance_core::welcomer::Welcomer;

mock! {
    FakeComplimentRepository{}
    impl ComplimentRepository for FakeComplimentRepository {
        fn fetch_compliment(&self) -> Sentence;
    }
}

#[test]
fn should_welcome_anyone() {
    let welcomer = setup_welcomer(setup_mock_compliment_repository(a_warm_compliment()));
    let welcome_message: String = welcomer.compute_default_message();

    assert_eq!(welcome_message, "Bonjour !");
}

#[test]
fn should_welcome_me_with_my_name() {
    let person_to_welcome = Person::with_name("Hervé", "Chasuble");

    let welcomer = setup_welcomer(setup_mock_compliment_repository(a_warm_compliment()));
    let welcome_message: String = welcomer.compute_personnalized_message(&person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Hervé Chasuble ! Vous êtes pimpant·e aujourd'hui !");
}

#[test]
fn welcome_message_should_greet_person_with_feminine_pronoun() {
    let female_gendered_person_to_welcome = Person::with_name_and_pronoun("Camille", "Diplodocus", Pronoun::Feminine);

    let welcomer = setup_welcomer(setup_mock_compliment_repository(a_warm_compliment()));
    let welcome_message = welcomer.compute_personnalized_message(&female_gendered_person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Camille Diplodocus ! Vous êtes pimpante aujourd'hui !")
}


#[test]
fn welcome_message_should_greet_person_with_masculine_pronoun() {
    let male_gendered_person_to_welcome = Person::with_name_and_pronoun("Francis", "Croustagnolle", Pronoun::Masculine);

    let welcomer = setup_welcomer(setup_mock_compliment_repository(a_warm_compliment()));
    let welcome_message = welcomer.compute_personnalized_message(&male_gendered_person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Francis Croustagnolle ! Vous êtes pimpant aujourd'hui !")
}

#[test]
fn smooth_talker_shall_rely_on_repository_inspiration() {
    let male_gendered_person_to_welcome = Person::with_name_and_pronoun("Baptiste", "Labrioche", Pronoun::Masculine);

    let mut repository = MockFakeComplimentRepository::new();

    repository.expect_fetch_compliment()
        .times(1)
        .return_once(|| another_warm_compliment());

    let welcomer = setup_welcomer(Box::new(repository));
    let welcome_message = welcomer.compute_personnalized_message(&male_gendered_person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Baptiste Labrioche ! Il n'est jamais trop tard pour ne pas perdre espoir, sois fort !")
}

fn setup_mock_compliment_repository(compliment_to_give: Sentence) -> Box<MockFakeComplimentRepository> {
    let mut repository = MockFakeComplimentRepository::new();

    repository.expect_fetch_compliment()
        .return_once(|| compliment_to_give);

    Box::new(repository)
}

fn setup_welcomer(compliment_repository: Box<MockFakeComplimentRepository>) -> Welcomer {
    Welcomer {
        smooth_talker: SmoothTalker {
            compliment_repository
        }
    }
}

fn a_warm_compliment() -> Sentence {
    Sentence {
        components: vec![
            Invariant(String::from("Vous êtes")),
            Variant {
                inclusive_form: String::from("pimpant·e"),
                masculine_form: String::from("pimpant"),
                feminine_form: String::from("pimpante"),
            },
            Invariant(String::from("aujourd'hui !"))
        ]
    }
}

fn another_warm_compliment() -> Sentence {
    Sentence {
        components: vec![
            Invariant(String::from("Il n'est jamais trop tard pour ne pas perdre espoir, sois")),
            Variant {
                inclusive_form: String::from("fort·e"),
                masculine_form: String::from("fort"),
                feminine_form: String::from("forte"),
            },
            Invariant(String::from("!"))
        ]
    }
}
