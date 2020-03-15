use ezcli::{flag, name::NameBuilder, named_flag};

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
    let args = ["--long-named-arg", "-s"];

    named_flag!(
        long_named_boolean,
        NameBuilder::new().long("long-named-arg").build(),
        args
    );
    named_flag!(
        short_named_boolean,
        NameBuilder::new().short("s").build(),
        args
    );

    assert!(long_named_boolean);
    assert!(short_named_boolean);
}

#[test]
fn should_enable_flag_of_long_and_short_named_arg() {
    let args = ["--both-named-arg", "-b"];

    named_flag!(
        both_named_boolean,
        NameBuilder::new().long("both-named-arg").short("b").build(),
        args
    );

    assert!(both_named_boolean);
}
