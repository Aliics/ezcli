use ezcli::arg;

#[test]
fn arguments() {
    arg!(-a: "yup");
    arg!(--my_arg: "yes");
    arg!(-a, --my_arg: "aha");
    println!("");

    arg!(-a <VALUE>: "yup");
    arg!(--my_arg <VALUE>: "yes");
    arg!(-a, --my_arg <VALUE>: "aha");
    println!("");

    arg!(-a [VAL]: "yup");
    arg!(--my_arg [VAL|VALUE]: "yes");
    arg!(-a, --my_arg [V|VAL|VALUE]: "aha");
    println!("");
}
