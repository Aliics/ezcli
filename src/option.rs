
#[macro_export]
/// Optional command line argument with associated value.
///
/// When provided via command line it will return `Some` wrapping the next argument
/// with it (e.g. args="program -a val" -> Some("val")). `None` will be returned
/// when the option is not provided or does not follow syntax. This macro replaces
/// underscores in long-name argument token with hyphens (e.g.: `option!(--my_arg)`
/// -> `"--my-arg"`).
/// ```
/// use ezcli::option;
///
/// // accepts "-a" (short-name form)
/// // if passed in, this expression is true
/// option!(-a);
/// // accepts "--my_arg" (long-name form)
/// // if passed in, this expression is true
/// option!(--my_arg);
/// ```
/// ------
/// Use the keyword `let` before the argument to create a variable with the same
/// name as the argument.
/// ```
/// use ezcli::option;
///
/// // Creates a variable named after the option
/// option!(let --my_arg);
/// ```
/// ------
/// You can use both *short-name* (-a) and *long-name forms* (--arg) together
/// by separating them with a comma.
/// ```
/// use ezcli::option;
///
/// option!(-a, --my_arg);
/// let arg_var = option!(-b, --my_arg);
/// ```
/// ------
/// In some case of not wanting to use the program's environment arguments
/// using a slice is also possible.
/// ```
/// use ezcli::option;
///
/// let args = ["--my_arg"];
/// option!(--my_arg, args);
/// ```
macro_rules! option {
    (-$short_name:tt) => {
        $crate::option::env_arg_val(format!("-{}", stringify!($short_name)))
    };
    (--$long_name:tt) => {
        $crate::option::env_arg_val(format!("--{}", stringify!($long_name).replace('_', "-")))
    };
    (let -$short_name:tt) => {
        let $short_name: Option<String> = $crate::option!(-$short_name);
    };
    (let --$long_name:tt) => {
        let $long_name: Option<String> = $crate::option!(--$long_name);
    };
    (-$short_name:tt, --$long_name:tt) => {
        $crate::option!(-$short_name).or($crate::option!(--$long_name))
    };

    (-$short_name:tt, $args:ident) => {
        $crate::option::arg_val(format!("-{}", stringify!($short_name)), &$args)
    };

    (--$long_name:tt, $args:ident) => {
        $crate::option::arg_val(
            format!("--{}", stringify!($long_name).replace('_', "-")),
            &$args,
        )
    };
    (let -$short_name:tt, $args:ident) => {
        let $short_name: Option<&str> = $crate::option!(-$short_name, $args);
    };
    (let --$long_name:tt, $args:ident) => {
        let $long_name: Option<&str> = $crate::option!(--$long_name, $args);
    };
    (-$short_name:tt, --$long_name:tt, $args:ident) => {
        $crate::option!(-$short_name, $args).or($crate::option!(--$long_name, $args))
    };
}

/// Get the string value of an argument present in the `std::env::args()`.
/// Argument name must have the prepended hyphens (e.g. -a, --my_arg).
pub fn env_arg_val(name: String) -> Option<String> {
    let mut iter = std::env::args().into_iter().skip(1);
    iter.position(|arg| arg == name).and_then(|_| iter.next())
}

/// Get the string value of an argument provided by the args parameter.
/// Argument name must have the prepended hyphens (e.g. -a, --my_arg).
pub fn arg_val<'a>(name: String, args: &[&'a str]) -> Option<&'a str> {
    let mut iter = args.into_iter().skip(1);
    iter.position(|&arg| arg == name)
        .and_then(|_| iter.next().map(|&a| a))
}
