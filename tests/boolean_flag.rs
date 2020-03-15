use ezcli::{flag, name::Name, named_flag};

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

#[test]
fn should_enable_named_flag_when_arg_given_with_long_and_short_name() {
    let mut args = ["--long-named-arg", "-s"];

    named_flag!(long_named_boolean, Name::long("long-named-arg"), args);
    named_flag!(short_named_boolean, Name::short("s"), args);

    assert!(long_named_boolean);
    assert!(short_named_boolean);
}

#[test]
fn should_enable_flag_of_long_and_short_named_arg() {
    let mut args = ["--both-named-arg", "-b"];

    named_flag!(both_named_boolean, Name::new("both-named-arg", "b"), args);

    assert!(both_named_boolean);
}

#[test]
fn should_not_enable_flag_of_long_and_short_named_arg_not_given() {
    named_flag!(not_enabled, Name::new("both-named-arg", "b"));

    assert!(!not_enabled);
}
