use ezcli::optional_arg;

#[test]
fn should_return_some_when_argument_provided() {
    let args = ["--my_arg", "value"];

    optional_arg!(my_arg, args);

    assert_eq!("value", my_arg.unwrap());
}
