use bienveillance_core::person::Person;
use bienveillance_core::person::Pronoun;
use bienveillance_core::welcomer::Welcomer;

#[test]
fn should_welcome_anyone() {
    let welcomer = Welcomer;
    let welcome_message: String = welcomer.compute_default_message();

    assert_eq!(welcome_message, "Bonjour !");
}

#[test]
fn should_welcome_me_with_my_name() {
    let person_to_welcome = Person::with_name("Hervé", "Chasuble");

    let welcomer = Welcomer;
    let welcome_message: String = welcomer.compute_personnalized_message(&person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Hervé Chasuble ! Vous êtes pimpant·e aujourd'hui !");
}

#[test]
fn welcome_message_should_greet_female_pronouned_person() {
    let female_pronouned_person_to_welcome = Person::with_name_and_pronoun("Camille", "Diplodocus", Pronoun::Feminine);

    let welcomer = Welcomer;
    let welcome_message = welcomer.compute_personnalized_message(&female_pronouned_person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Camille Diplodocus ! Vous êtes pimpante aujourd'hui !")
}

#[test]
fn welcome_message_should_greet_masculine_pronouned_person() {
    let masculine_pronouned_person_to_welcome = Person::with_name_and_pronoun("Francis", "Croustagnolle", Pronoun::Masculine);

    let welcomer = Welcomer;
    let welcome_message = welcomer.compute_personnalized_message(&masculine_pronouned_person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Francis Croustagnolle ! Vous êtes pimpant aujourd'hui !")
}
