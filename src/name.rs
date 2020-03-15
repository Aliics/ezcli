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
        let $name = $args
            .iter()
            .find(|s| {
                if $named.long.is_some() {
                    return **s == format!("--{}", $named.long.unwrap());
                }

                if $named.short.is_some() {
                    return **s == format!("-{}", $named.short.unwrap());
                }

                false
            })
            .is_some();
    };
}
