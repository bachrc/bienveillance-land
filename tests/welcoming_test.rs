use bienveillance_core::welcomer::Welcomer;

#[test]
fn should_welcome_anyone() {
    let welcome_message: String = Welcomer::compute_welcome_message();

    assert_eq!(welcome_message, "Bonjour !")
}
