/// Struct represents a long and/or short name from the command line.
///
/// Both long and short are option string for what to be accepted, but
/// one of them must be provided to yield results. If the variable name
/// is to be used instead, then refer to [`flag`] or [`option`].
///
/// [`flag`]: ./macro.flag.html
/// [`option`]: ./macro.option.html
pub struct Name {
    pub long: Option<String>,
    pub short: Option<String>,
}

impl Name {
    /// Create a [`Name`] with a long and a short name as the parameters.
    ///
    /// **Accepts**: --long-name *or* -s
    ///
    /// [`Name`]: ./struct.Name.html
    pub fn new(long: &str, short: &str) -> Self {
        Self {
            long: Some(long.to_string()),
            short: Some(short.to_string()),
        }
    }

    /// Create a [`Name`] with just a long name.
    ///
    /// **Accepts:** --long-name
    ///
    /// [`Name`]: ./struct.Name.html
    pub fn long(name: &str) -> Self {
        Self {
            long: Some(name.to_string()),
            short: None,
        }
    }

    /// Create a [`Name`] with just a short name.
    ///
    /// **Accepts:** -s
    ///
    /// [`Name`]: ./struct.Name.html
    pub fn short(name: &str) -> Self {
        Self {
            long: None,
            short: Some(name.to_string()),
        }
    }
}

/// Command line argument macro for named flags.
///
/// The [`flag`] macro does not allow for an alias over the variable name
/// already given. This macro allows you to pass a [`Name`] in as a parameter
/// to create flags with a long and short name variant.
/// ```
/// use ezcli::{named_flag, name::Name};
///
/// // accepts "--cool-flag"
/// // if passed in, "my_flag" will be true
/// named_flag!(my_flag, Name::long("cool-flag"));
/// ```
/// Also allows for a slice of args to be passed in.
/// ```
/// use ezcli::{named_flag, name::Name};
///
/// let args = ["f"];
///
/// // accepts "-f"
/// // if passed in, "flag" will be true
/// named_flag!(flag, Name::short("f"));
/// ```
///
/// [`flag`]: ./macro.flag.html
/// [`Name`]: ./name/struct.Name.html
#[macro_export]
macro_rules! named_flag {
    ($name:tt, $named:expr, $args:ident) => {
        let args: Vec<String> = $args.iter().map(|s| s.to_string()).collect();
        let $name = $crate::name::_named_flag($named, args.as_slice());
    };
    ($name:tt, $named:expr) => {
        let $name = {
            let args: Vec<String> = std::env::args().collect();
            $crate::name::_named_flag($named, args.as_slice())
        };
    };
}

/// Named optional command line argument with associated value.
///
/// Functionally identical to [`option`] but accepts a [`Name`] to allow for
/// more robust CLI naming options. If there is no provided value with the
/// option, the created variable is `None`, otherwise it is equal to the
/// value wrapped in `Some`.
/// ```
/// use ezcli::{named_option, name::Name};
///
/// // accepts "--amazing-option"
/// // "my_option" is now an Option<String> of the value passed in
/// named_option!(my_option, Name::long("amazing-option"));
/// ```
/// In some case of not wanting to use the program's environment arguments
/// using a slice is also possible.
/// ```
/// use ezcli::{named_option, name::Name};
///
/// let args = ["-b", "value"];
///
/// // accepts "--accepts-both" or "-b"
/// // "my_option" is now an Option<String> of the value passed in
/// named_option!(my_option, Name::new("accepts-both", "b"), args);
/// ```
#[macro_export]
macro_rules! named_option {
    ($name:tt, $named:expr, $args:ident) => {
        let $name = {
            let args: Vec<String> = $args.iter().map(|s| s.to_string()).collect();
            $crate::name::_named_option($named, args.as_slice())
        };
    };

    ($name:tt, $named:expr) => {
        let $name = {
            let args: Vec<String> = std::env::args().collect();
            $crate::name::_named_option($named, args.as_slice())
        };
    };
}

pub fn _named_flag(name: Name, args: &[String]) -> bool {
    args.iter()
        .find(|s| {
            **s == format!("--{}", name.long.clone().unwrap_or_default())
                || **s == format!("-{}", name.short.clone().unwrap_or_default())
        })
        .is_some()
}

pub fn _named_option(name: Name, args: &[String]) -> Option<String> {
    let mut optional = None;
    let wanted_long = format!("--{}", name.long.unwrap_or_default());
    let wanted_short = format!("-{}", name.short.unwrap_or_default());
    for i in 0..args.len() {
        if (args[i] == wanted_long || args[i] == wanted_short) && args.len() > i + 1 {
            optional = Some(args[i + 1].clone());
            break;
        }
    }

    optional
}
