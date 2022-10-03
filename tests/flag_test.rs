use ezcli::flag;

#[test]
fn should_enable_boolean_flag_when_arg_given() {
    let args = ["program", "-b", "--my-boolean"];

    flag!(let -b, args);
    flag!(let --my_boolean, args);
    assert!(b);
    assert!(my_boolean);

    assert!(flag!(-b, args));
    assert!(flag!(--my_boolean, args));
    assert!(flag!(-b, --my_boolean, args));
}

#[test]
fn should_not_enable_flag_when_no_arg_given() {
    flag!(let -b);
    flag!(let --my_boolean);
    assert!(!b);
    assert!(!my_boolean);

    assert!(!flag!(-b));
    assert!(!flag!(--my_boolean));
    assert!(!flag!(-b, --my_boolean));
}

#[test]
fn flag_group() {
    let args = ["program", "-abcd", "--efgh"];

    assert!(flag!(-a, args));
    assert!(flag!(-b, args));
    assert!(flag!(-c, args));
    assert!(flag!(-d, args));

    assert!(!flag!(-e, args));
    assert!(!flag!(-f, args));
    assert!(!flag!(-g, args));
    assert!(!flag!(-h, args));
}
