# ezcli

Small and lightweight enough to be put into any CLI application quickly with no fuss.
Just call a macro or two and it's up and running!

If you do want a more fleshed out CLI crate, you should check out 
[clap](https://crates.io/crates/clap).

# how to use

**ezcli** is pretty easy to use and nothing too crazy is happening.
Using *flag!* or *option!* will allow for them to be passed 
as CLI args and it even creates a nifty little variable for you to use once the macro 
has been called. **Note** that the macros will replace underscores (_) with hyphens (-) in the arguments name.

## flag

Command line argument for a boolean state. The `flag!` macro only requires a variable
name. Once invoked, it will check the command line arguments for a matching flag.
Optionally, passing the keyword `let` will create a variable with the same name as the flag.
```rust
use ezcli::flag;

if flag!(--my_boolean) { // "--my-boolean" exists in arguments
    // do stuff because flag is given
}

// OR

flag!(let --my_boolean);

if my_boolean { // "--my-boolean" exists in arguments
    // do stuff because flag is given
}
```

## option
Command line argument for an optional parameter. The `option!` macro requires a
variable name, like `flag!`, and will create a variable if `let` is included. The value will be the next argument wrapped in a `Some`, unless it is not provided then it'll be `None`. 
```rust
use ezcli::option;

match option!(--my_arg) {
    Some(x) => {}, // use x
    None => {}, // handle no value
}

// OR 

option!(let --my_arg);

match my_arg {
    Some(x) => {}, // use x
    None => {}, // handle no value
}
```

## short- and long-name arguments
A command line argument can be in short-name (indicated by 1 hyphen: -a) or long-name (indicated by 2 hyphens: --arg). This applies to both `flag!` and `option!`.
```rust
use ezcli::{flag, option};

if flag!(-f, --flag) { 
    // "-f" or "--flag" exists in arguments
}

// "-a" or "--arg" exists in arguments
match option!(-a, --arg) {
    Some(x) => {},
    None => {},
}

// OR

let my_flag = flag!(-f, --flag);
...
```

<hr>

See [flag](tests/boolean_flag.rs) and [option](tests/optional_args.rs) tests for more detailed examples.
