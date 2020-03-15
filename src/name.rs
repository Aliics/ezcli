pub struct Name {
    pub long: Option<String>,
    pub short: Option<String>,
}

pub struct NameBuilder(Name);

impl NameBuilder {
    pub fn new() -> Self {
        Self(Name {
            long: None,
            short: None,
        })
    }

    pub fn long(mut self, name: &str) -> Self {
        self.0.long = Some(name.to_string());
        self
    }

    pub fn short(mut self, name: &str) -> Self {
        self.0.short = Some(name.to_string());
        self
    }

    pub fn build(self) -> Name {
        self.0
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
