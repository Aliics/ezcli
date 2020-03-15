use ezcli::flag;

#[test]
fn should_enable_boolean_flag_when_arg_given() {
    let args = ["--my_boolean"];

    flag!(my_boolean, args);

    assert!(my_boolean);
}

#[test]
fn should_not_enable_flag_when_no_arg_given() {
    flag!(not_enabled);

    assert!(!not_enabled);
}
