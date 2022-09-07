#[macro_export]
/// Output documentation for your arguments. Write argument name followed by a
/// description. This macro replaces underscores in long-name argument token with
/// hyphens (e.g.: `flag!(--my_arg)`.
/// -> `"--my-arg"`)
/// ```
/// use ezcli::args;
/// 
/// // Flag (boolean) arguments
/// args!(-a, --my_arg: "Description");
/// // Option arguments
/// args!(-a, --my_arg <VALUE>: "Description");
/// // Option arguments with specific possible values
/// args!(-a, --my_arg [V|VAL|VALUE]: "Description");
/// ```
macro_rules! arg {
    // Flags (booleans)
    (-$short_arg:tt: $desc:expr) => {
        println!("-{}:\n    {}", stringify!($short_arg), $desc)
    };
    (--$long_arg:tt: $desc:expr) => {
        println!("--{}:\n    {}", stringify!($long_arg).replace('_', "-"), $desc)
    };
    (-$short_arg:tt, --$long_arg:tt: $desc:expr) => {
        println!("-{}, --{}:\n    {}", stringify!($short_arg), stringify!($long_arg).replace('_', "-"), $desc)
    };

    // Arguments with any value
    (-$short_arg:tt <$val:tt>: $desc:expr) => {
        println!("-{} <{}>:\n    {}", stringify!($short_arg), stringify!($val), $desc)
    };
    (--$long_arg:tt <$val:tt>: $desc:expr) => {
        println!("--{} <{}>:\n    {}", stringify!($long_arg).replace('_', "-"), stringify!($val), $desc)
    };
    (-$short_arg:tt, --$long_arg:tt <$val:tt>: $desc:expr) => {
        println!("-{}, --{} <{}>:\n    {}", stringify!($short_arg), stringify!($long_arg).replace('_', "-"), stringify!($val), $desc)
    };

    // Arguments with specific optional values
    (-$short_arg:tt [$( $opt_val:tt )|*]: $desc:expr) => {
        print!("-{} [", stringify!($short_arg));
        $(
            print!("{}|", stringify!($opt_val));
        )*
        print!("{}", 08 as char); // removes the trailing pipe |
        println!("]:\n    {}", $desc)
    };
    (--$long_arg:tt [$( $opt_val:tt )|*]: $desc:expr) => {
        print!("--{} [", stringify!($long_arg).replace('_', "-"));
        $(
            print!("{}|", stringify!($opt_val));
        )*
        print!("{}", 08 as char); // removes the trailing pipe |
        println!("]:\n    {}", $desc)
    };
    (-$short_arg:tt, --$long_arg:tt [$( $opt_val:tt )|*]: $desc:expr) => {
        print!("-{}, --{} [", stringify!($short_arg).replace('_', "-"), stringify!($long_arg).replace('_', "-"));
        $(
            print!("{}|", stringify!($opt_val));
        )*
        print!("{}", 08 as char); // removes the trailing pipe |
        println!("]:\n    {}", $desc)
    };
}