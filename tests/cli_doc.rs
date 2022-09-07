use ezcli::arg;

#[test]
fn arguments() {
    // Flags (booleans)
    arg!(-a: "yup");
    arg!(--my_arg: "yes");
    arg!(-a, --my_arg: "aha");
    println!("");

    // Arguments with any value
    arg!(-a <VALUE>: "yup");
    arg!(--my_arg <VALUE>: "yes");
    arg!(-a, --my_arg <VALUE>: "aha");
    println!("");

    // Arguments with specific optional values
    arg!(-a [VAL]: "yup");
    arg!(--my_arg [VAL|VALUE]: "yes");
    arg!(-a, --my_arg [V|VAL|VALUE]: "aha");
    println!("");
}