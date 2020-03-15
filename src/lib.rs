pub mod name;

/// Command line argument flag for on/off state.
///
/// Uses `std::env:args()` to determine the arguments passed to the program.
/// If there is an argument matching the flag's name then this variable will be
/// truthy.
/// ```
/// use ezcli::flag;
///
/// flag!(my_boolean);
/// ```
/// In some case of not wanting to use the program's environment arguments
/// using a slice is also possible.
/// ```
/// use ezcli::flag;
///
/// let args = ["--my_boolean"];
/// flag!(my_boolean, args);
/// ```
/// If the command line argument name should be different to the variable name
/// then use [`named_flag`].
///
/// [`named_flag`]: ./macro.named_flag.html
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

/// Optional command line argument with associated value.
///
/// When provided via command line it will return `Some` wrapping the value
/// passed along with it. `None` will be returned when the option is not
/// provided or does not follow syntax.
/// ```
/// use ezcli::option;
///
/// option!(my_option);
/// ```
/// In some case of not wanting to use the program's environment arguments
/// using a slice is also possible.
/// ```
/// use ezcli::option;
///
/// let args = ["--my_option", "value"];
/// option!(my_option, args);
/// ```
#[macro_export]
macro_rules! option {
    ($name:tt, $args:ident) => {
        let $name: Option<String> = {
            let args: Vec<String> = $args.iter().map(|s| s.to_string()).collect();
            $crate::_option(stringify!($name), args.as_slice())
        };
    };
    ($name:tt) => {
        let $name: Option<String> = {
            let args: Vec<String> = std::env::args().collect();
            $crate::_option(stringify!($name), args.as_slice())
        };
    };
}

pub fn _option(name: &str, args: &[String]) -> Option<String> {
    let mut optional = None;
    let wanted_arg = format!("--{}", name);
    for i in 0..args.len() {
        if args[i] == wanted_arg && args.len() > i + 1 {
            optional = Some(args[i + 1].clone());
            break;
        }
    }

    optional
}
