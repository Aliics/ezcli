use ezcli::{name::Name, named_option, option};

#[test]
fn should_return_some_when_argument_provided() {
    let args = ["--my_arg", "value"];

    option!(my_arg, args);

    assert_eq!("value", my_arg.unwrap());
}

#[test]
fn should_return_none_when_no_arguments_provided() {
    option!(will_be_none);

    assert!(will_be_none.is_none());
}

#[test]
fn should_return_two_somes_when_given_two_arguments() {
    let args = ["--my_arg0", "big", "--my_arg1", "wuv"];

    option!(my_arg0, args);
    option!(my_arg1, args);

    assert_eq!("big", my_arg0.unwrap());
    assert_eq!("wuv", my_arg1.unwrap());
}

#[test]
fn should_return_some_when_argument_when_named_is_provided() {
    let args = ["--good-boy", "ollie"];

    named_option!(bunny, Name::long("good-boy"), args);

    assert_eq!("ollie", bunny.unwrap());
}

#[test]
fn should_return_some_when_argument_when_short_named_is_provided() {
    let args = ["-b", "candi"];

    named_option!(bunny, Name::short("b"), args);

    assert_eq!("candi", bunny.unwrap());
}

#[test]
fn should_return_some_when_argument_when_short_named_is_provided_on_name_accepting_either() {
    let args = ["-g", "alfie"];

    named_option!(bunny, Name::new("good-boy", "g"), args);

    assert_eq!("alfie", bunny.unwrap());
}

#[test]
fn should_return_none_when_no_named_argument_given() {
    named_option!(bunny, Name::long("alfie"));

    assert!(bunny.is_none());
}
