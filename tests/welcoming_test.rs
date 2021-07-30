use bienveillance_core::person::{Person, Pronoun};
use bienveillance_core::welcomer::Welcomer;

#[test]
fn should_welcome_anyone() {
    let welcomer = Welcomer;
    let welcome_message: String = welcomer.compute_default_welcome_message();

    assert_eq!(welcome_message, "Bonjour !");
}

#[test]
fn should_welcome_me_with_my_name() {
    let person_to_welcome = Person::with_name("Hervé", "Chasuble");

    let welcomer = Welcomer;
    let welcome_message: String = welcomer.compute_personnalized_welcome_message(&person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Hervé Chasuble ! Vous êtes pimpant.e aujourd'hui !");
}

#[test]
fn welcome_message_should_greet_feminine_pronouned_person() {
    let feminine_pronouned_person_to_welcome = Person::with_name_and_pronoun("Camille", "Diplodocus", Pronoun::Feminine);

    let welcomer = Welcomer;
    let welcome_message = welcomer.compute_personnalized_welcome_message(&feminine_pronouned_person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Camille Diplodocus ! Vous êtes pimpante aujourd'hui !")
}
