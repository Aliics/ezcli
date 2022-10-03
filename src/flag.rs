/// Command line argument flag for on/off state.
///
/// Uses `std::env:args()` to determine the arguments passed to the program.
/// If there is an argument matching the flag's name then this variable will
/// be truthy. There are two flag types that are often passed in cli arguments:
/// *short-name form* (-f) and *long-name form* (--flag). This macro replaces
/// underscores in long-name argument token with hyphens (e.g.:
/// `flag!(--my_boolean)` -> `"--my-boolean"`).
///
/// ```
/// use ezcli::flag;
/// // args: "-b --my-boolean"
///
/// // accepts "-b" (short-name form)
/// // if passed in, this expression is true
/// flag!(-b);
/// // accepts "--my_boolean" (long-name form)
/// // if passed in, this expression is true
/// flag!(--my_boolean);
/// ```
/// ------
/// Use the keyword `let` before the argument to create a variable with the same
/// name as the argument.
/// ```
/// use ezcli::flag;
///
/// // Creates a variable named after the flag
/// flag!(let --my_boolean);
/// ```
/// ------
/// You can use both *short-name* (-f) and *long-name forms* (--flag) together
/// by separating them with a comma.
/// ```
/// use ezcli::flag;
///
/// flag!(-b, --my_boolean);
/// let bool_var = flag!(-b, --my_boolean);
/// ```
/// ------
/// In some case of not wanting to use the program's environment arguments
/// using a slice is also possible.
/// ```
/// use ezcli::flag;
///
/// let args = ["--my_boolean"];
/// flag!(--my_boolean, args);
/// ```
/// ------
///  Flag groups can be used with short-name flags
/// ```
/// use ezcli::flag;
///
/// let args = ["-abc"];
/// flag!(-a, args);
/// flag!(-b, args);
/// ```
#[macro_export]
macro_rules! flag {
    (-$short_name:tt) => {
        $crate::flag::has_short_name_flag(std::env::args().into_iter(), stringify!($short_name))
    };
    (--$long_name:tt) => {
        $crate::flag::has_long_name_flag(std::env::args().into_iter(), stringify!($long_name))
    };

    (let -$short_name:tt) => {
        let $short_name: bool = $crate::flag!(-$short_name);
    };
    (let --$long_name:tt) => {
        let $long_name: bool = $crate::flag!(--$long_name);
    };
    (-$short_name:tt, --$long_name:tt) => {
        $crate::flag!(-$short_name) || $crate::flag!(--$long_name)
    };

    (-$short_name:tt, $args:ident) => {
        $crate::flag::has_short_name_flag(
            (&$args).into_iter().map(|a| a.to_string()),
            stringify!($short_name),
        )
    };
    (--$long_name:tt, $args:ident) => {
        $crate::flag::has_long_name_flag(
            (&$args).into_iter().map(|a| a.to_string()),
            stringify!($long_name),
        )
    };
    (let -$short_name:tt, $args:ident) => {
        let $short_name: bool = $crate::flag!(-$short_name, $args);
    };
    (let --$long_name:tt, $args:ident) => {
        let $long_name: bool = $crate::flag!(--$long_name, $args);
    };
    (-$short_name:tt, --$long_name:tt, $args:ident) => {
        $crate::flag!(-$short_name, $args) || $crate::flag!(--$long_name, $args)
    };
}

/// Check if the short-name version of the flag is present in the args.
pub fn has_short_name_flag(args: impl Iterator<Item = String>, name: &str) -> bool {
    args.skip(1)
        .find(|s| {
            s.chars().nth(0) == Some('-') && s.chars().nth(1) != Some('-') && s.contains(&name)
        })
        .is_some()
}

/// Check if the long-name version of the flag is present in the args.
pub fn has_long_name_flag(args: impl Iterator<Item = String>, name: &str) -> bool {
    args.skip(1)
        .find(|s| **s == format!("--{}", name.replace('_', "-")))
        .is_some()
}
