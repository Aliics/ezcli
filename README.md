# ezcli
Small and lightweight to be in any command line application, without having to worry
about a lot of extra nonsense. I intend for this to be designed with simplicity in mind.

# flag
Command line argument for on/off state. Using the `flag` macro you pass a variable
name in and it is now available to that scope. The variables value is determined
on whether or not a CLI option is passed in with the same name.
```rust
use ezcli::flag;

// my_boolean is true if program args contain "--my_boolean"
flag!(my_boolean); 
```

# option
Command line argument for an optional parameter. Using the `option` macro you pass
a variable name in making it available to that scope. Takes the value provided to
it from the CLI if available. When available it will be a `Some` wrapping that value and
when not, it'll be `None`.
```rust
use ezcli::option;

// my_arg is Some(x) if given "--my_arg x", otherwise None 
option!(my_arg);
```

# named_flag
Command line argument for on/off state. Using the `named_flag` macro you pass a 
variable name in and it is now available to that scope. The variables value 
is determined on whether or not a CLI option is passed in with a given name.
```rust
use ezcli::{named_flag, name::Name};

// my_boolean is available to the program but accepts "-b" or "--my-boolean" 
named_flag!(my_boolean, Name::new("my-boolean", "b"));
```

# named_option
Command line argument for a named optional parameter. Using `named_option` macro you
pass in a variable, which will be created, and a `Name` struct that defines the long
and short naming. Takes the value provided to it from the CLI if available. When 
available it will be a `Some` wrapping that value and when not, it'll be `None`.
```rust
use ezcli::{named_option, name::Name};

named_option!(my_option, Name::new("my-option", "o"));
```
