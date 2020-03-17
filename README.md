# ezcli
Small and lightweight enough to be put into any CLI application quickly with no fuss.
Just call a macro or two and it's up and running!

If you do want a more fleshed out CLI crate, you should check out 
[clap](https://crates.io/crates/clap).

# flag
Command line argument for a boolean state. The `flag!` macro only requires a variable
name. Once, invoked a variable of the passed name is created and it will check the
command line for matching flags.
```rust
use ezcli::flag;

flag!(my_boolean);

if my_boolean { // "--my_boolean" exists in arguments
    // do stuff because flag is given
}
```

# option
Command line argument for an optional parameter. The `option!` macro requires a
variable name, like `flag!`, and makes it available to the scope. The value of the
new variable will be a `Some`, unless it is not provided then it'll be `None`.
```rust
use ezcli::option;

option!(my_arg);

match my_arg {
    Some(x) => {}, // use x
    None => {}, // handle no value
}
```

# named_flag
Command line argument mimicking the behaviour of the `flag!` macro. The only
difference is that it accepts a `Name`, which will make short and long CLI names.
```rust
use ezcli::{named_flag, name::Name};

// --my-boolean and -b are accepted
named_flag!(my_boolean, Name::new("my-boolean", "b")); 
```

# named_option
Like how the `flag!` and `named_flag!` macros are similar, this one is a nameable
version of the `option!` macro. 
```rust
use ezcli::{named_option, name::Name};

// --my-option and -o are accepted
named_option!(my_option, Name::new("my-option", "o"));
```
