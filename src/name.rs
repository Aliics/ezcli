pub struct Name {
    pub long: Option<String>,
    pub short: Option<String>,
}

impl Name {
    pub fn new(long: &str, short: &str) -> Self {
        Self {
            long: Some(long.to_string()),
            short: Some(short.to_string()),
        }
    }

    pub fn long(name: &str) -> Self {
        Self {
            long: Some(name.to_string()),
            short: None,
        }
    }

    pub fn short(name: &str) -> Self {
        Self {
            long: None,
            short: Some(name.to_string()),
        }
    }
}

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
