# ezcli
Small and lightweight to be in any command line application, without having to worry
about a lot of extra nonsense. I intend for this to be designed with simplicity in mind.

# flag
Command line argument for on/off state. Using the `flag` macro you pass a variable
name in and it is now available to that scope. The variables value is determined
on whether or not a CLI option is passed in with the same name.
```rust
use ezcli::flag;

flag!(my_boolean); // my_boolean is true if program args contain --my_boolean
```
