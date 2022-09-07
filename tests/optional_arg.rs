use ezcli::{option/*, name::Name, named_option*/};

#[test]
fn should_return_some_when_argument_provided() {
    // TODO: what should happen when both the short- and long-name arguments are passed? should the first or last occurrence be used?
    let args = ["program", "-a", "val", "--my-arg", "value"];

    // Create variables
    option!(let -a, args);
    option!(let --my_arg, args);
    assert_eq!(a, Some("val"));
    assert_eq!(my_arg, Some("value"));

    // Use expressions
    assert_eq!(option!(-a, args), Some("val"));
    assert_eq!(option!(--my_arg, args), Some("value"));
    assert_eq!(option!(-a, --my_arg, args), Some("val"));
}

#[test]
fn should_return_none_when_no_arguments_provided() {
    // Create variables
    option!(let -a);
    option!(let --my_arg);
    assert_eq!(a, None);
    assert_eq!(my_arg, None);

    // Use expressions
    assert_eq!(option!(-a), None);
    assert_eq!(option!(--my_arg), None);
    assert_eq!(option!(-a, --my_arg), None);
}