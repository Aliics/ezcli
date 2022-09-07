pub mod doc;

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
    // -- Use the arguments passed directly to the program by the Command Line
    (-$short_name:tt) => {
        std::env::args()
            .into_iter()
            .skip(1) // the program name is always the 0th argument, so it can be skipped
            // check that the char of the flag is in a flag group
            .find(|s|
                s.chars().nth(0) == Some('-') && s.chars().nth(1) != Some('-') &&
                s.contains(stringify!($short_name))
            ).is_some()
    };
    (--$long_name:tt) => {
        std::env::args()
            .into_iter()
            .skip(1)
            .find(|s| *s == format!("--{}", stringify!($long_name).replace('_', "-")))
            .is_some()
    };
    // Prepend the argument with `let` to create a variable with the same name
    (let -$short_name:tt) => {
        let $short_name: bool = $crate::flag!(-$short_name);
    };
    (let --$long_name:tt) => {
        let $long_name: bool = $crate::flag!(--$long_name);
    };
    // Use both short-name and long-name
    (-$short_name:tt, --$long_name:tt) => {
        $crate::flag!(-$short_name) || $crate::flag!(--$long_name)
    };

    
    // -- Use arguments defined in a slice
    // TODO: maybe use a different syntax for when args is custom defined (something like flag!(args; let -b, --my_bool))
    (-$short_name:tt, $args:ident) => {
        $args.iter()
            .skip(1) // the program name is always the 0th argument, so it can be skipped
            // check that the char of the flag is in a flag group
            .find(|s|
                s.chars().nth(0) == Some('-') && s.chars().nth(1) != Some('-') &&
                s.contains(stringify!($short_name))
            ).is_some()
    };
    (--$long_name:tt, $args:ident) => {
        $args.iter()
            .skip(1)
            .find(|s| **s == format!("--{}", stringify!($long_name).replace('_', "-")))
            .is_some()
    };
    // Prepend the argument with `let` to create a variable with the same name
    (let -$short_name:tt, $args:ident) => {
        let $short_name: bool = $crate::flag!(-$short_name, $args);
    };
    (let --$long_name:tt, $args:ident) => {
        let $long_name: bool = $crate::flag!(--$long_name, $args);
    };
    // Use both short-name and long-name
    (-$short_name:tt, --$long_name:tt, $args:ident) => {
        $crate::flag!(-$short_name, $args) || $crate::flag!(--$long_name, $args)
    };
}

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
#[macro_export]
macro_rules! option {
    // -- Use the arguments passed directly to the program by the Command Line
    (-$short_name:tt) => {
        $crate::env_arg_val(format!("-{}", stringify!($short_name)))
    };
    (--$long_name:tt) => {
        $crate::env_arg_val(format!("--{}", stringify!($long_name).replace('_', "-")))
    };
    // Prepend the argument with `let` to create a variable with the same name
    (let -$short_name:tt) => {
        let $short_name: Option<String> = $crate::option!(-$short_name);
    };
    (let --$long_name:tt) => {
        let $long_name: Option<String> = $crate::option!(--$long_name);
    };
    // Use both short-name and long-name
    (-$short_name:tt, --$long_name:tt) => {
        // if short-name was found use that one, if not check long-name
        match $crate::option!(-$short_name) {
            None => $crate::option!(--$long_name),
            some => some
        }
    };


    // -- Use arguments defined in a slice
    // TODO: maybe use a different syntax for when args is custom defined (something like flag!(args; let -a, --my_arg))
    (-$short_name:tt, $args:ident) => {
        $crate::arg_val(format!("-{}", stringify!($short_name)), &$args)
    };
    (--$long_name:tt, $args:ident) => {
        $crate::arg_val(format!("--{}", stringify!($long_name).replace('_', "-")), &$args)
    };
    // Prepend the argument with `let` to create a variable with the same name
    (let -$short_name:tt, $args:ident) => {
        let $short_name: Option<&str> = $crate::option!(-$short_name, $args);
    };
    (let --$long_name:tt, $args:ident) => {
        let $long_name: Option<&str> = $crate::option!(--$long_name, $args);
    };
    // Use both short-name and long-name
    (-$short_name:tt, --$long_name:tt, $args:ident) => {
        // if short-name was found use that one, if not check long-name
        match $crate::option!(-$short_name, $args) {
            None => $crate::option!(--$long_name, $args),
            some => some,
        }
    };
}


/// Get the string value of an argument. The value is the argument directly after
/// the specified argument ("--arg val"). <br>
/// Argument name must have the prepended hyphens (e.g. --my_bool, -b).
pub fn arg_val<'a>(name: String, args: &[&'a str]) -> Option<&'a str> {
    // the program name is always the 0th argument, so it can be skipped
    let mut iter = args.iter().skip(1);

    while let Some(&arg) = iter.next() {
        if arg == name {
            return match iter.next() {
                // Argument and value found
                Some(&next_arg) => Some(next_arg),
                // There is no next arg
                None => None
            }
        }
    }

    // argument was not found
    None
}

// TODO: unify:... use either String or &str, not both
/// Get the string value of an argument. The value is the argument directly after
/// the specified argument ("--arg val"). <br>
/// Argument name must have the prepended hyphens (e.g. -a, --my_arg).
pub fn env_arg_val(name: String) -> Option<String> {
    // the program name is always the 0th argument, so it can be skipped
    let mut iter = std::env::args().into_iter().skip(1);

    while let Some(arg) = iter.next() {
        if arg == name {
            return match iter.next() {
                // There is no next arg
                None => None,
                // Argument and value found
                some => some,
            }
        }
    }

    // argument was not found
    None
}