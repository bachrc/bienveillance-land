use bienveillance_core::person::Person;
use bienveillance_core::welcomer::Welcomer;

#[test]
fn should_welcome_anyone() {
    let welcome_message: String = Welcomer::compute_welcome_message();

    assert_eq!(welcome_message, "Bonjour !")
}

#[test]
fn should_welcome_me_with_my_name() {
    let person_to_welcome = Person::with_name("Hervé", "Chasuble");

    let welcome_message: String = Welcomer::compute_personnalized_welcome_message(&person_to_welcome);

    assert_eq!(welcome_message, "Bonjour Hervé Chasuble ! Vous êtes pimpant.e aujourd'hui !");
}
