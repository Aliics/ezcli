/// Command line argument flag for on/off state.
///
/// Uses `std::env:args()` to determine the arguments
/// passed to the program. If there is an argument matching
/// the flag's name then this variable will be truthy.
/// ```
/// use ezcli::flag;
///
/// flag!(my_boolean);
/// ```
/// In some case of not wanting to use the program's environment
/// arguments using a slice is also possible.
/// ```
/// use ezcli::flag;
///
/// let args = ["--my_boolean"];
/// flag!(my_boolean, args);
/// ```
#[macro_export]
macro_rules! flag {
    ($name:tt, $args:ident) => {
        let $name: bool = $args
            .iter()
            .find(|s| **s == format!("--{}", stringify!($name)))
            .is_some();
    };
    ($name:tt) => {
        let $name: bool = std::env::args()
            .into_iter()
            .find(|s| *s == format!("--{}", stringify!($name)))
            .is_some();
    };
}

#[macro_export]
macro_rules! optional_arg {
    ($name:tt, $args:ident) => {
        let $name: Option<&str> = {
            let mut i = 0;
            let mut optional = None;
            let wanted_arg = format!("--{}", stringify!(my_arg));
            while (i < $args.len()) {
                if $args[i] == wanted_arg && $args.len() > i + 1 {
                    optional = Some($args[i + 1]);
                    break;
                }

                i += 1;
            }

            optional
        };
    };
}
