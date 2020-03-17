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
    /// [`Name`]: ./struct.Name.html
    pub fn new(long: &str, short: &str) -> Self {
        Self {
            long: Some(long.to_string()),
            short: Some(short.to_string()),
        }
    }

    /// Create a [`Name`] with just a long name.
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
/// // Macro creates variable called my_flag.
/// // Accepts --cool-flag to be passed via CLI.
/// named_flag!(my_flag, Name::long("cool-flag"));
/// ```
/// Also allows for a slice of args to be passed in.
/// ```
/// use ezcli::{named_flag, name::Name};
///
/// let mut args = ["f"];
///
/// // Macros creates variable called flag.
/// // Accepts -f as a short argument.
/// named_flag!(flag, Name::short("f"));
/// ```
///
/// [`flag`]: ./macro.flag.html
/// [`Name`]: ./name/struct.Name.html
#[macro_export]
macro_rules! named_flag {
    ($name:tt, $named:expr, $args:ident) => {
        let $name = $crate::name::_named_flag($named, &mut $args);
    };
    ($name:tt, $named:expr) => {
        let $name = {
            let mut args = std::env::args().collect::<Vec<String>>();
            let mut args_str = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            $crate::name::_named_flag($named, args_str.as_slice())
        };
    };
}

#[macro_export]
macro_rules! named_option {
    ($name:tt, $named:expr, $args:ident) => {
        let $name = {
            let mut args: Vec<String> = $args.iter().map(|s| s.to_string()).collect();
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

pub fn _named_flag(name: Name, args: &[&str]) -> bool {
    args.iter()
        .find(|s| {
            if name.long.is_some() {
                return **s == format!("--{}", name.long.clone().unwrap());
            }

            if name.short.is_some() {
                return **s == format!("-{}", name.short.clone().unwrap());
            }

            false
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
