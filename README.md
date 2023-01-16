# 🗃 arguments-parser

## How to use
 
```rust
use std::env;
use args::Args;

fn main() {
    // i# - type i32
    // s* - type &str
    // b  - type bool
    let arg = Args::new("i#,s*,b", env::args());

    if arg.get_i32("i") == 1 {
        ...
    }
    if arg.get_str("s") == "..." {
        ...
    }
    if arg.get_bool("b") {
        ...
    }
}
```
