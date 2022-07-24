# diff-ba-rs

## About

This library provides macros to get the difference of variables caused by a procedure.

Before and after the operation, use the pretty-print format by fmt::Debug to string them, and get their diff.

## How to use

```toml
# Cargo.toml

[dependencies]
diff-ba-rs = "0.1.0"
```

```rust
use diff_ba_rs::prelude::*;

#[derive(Debug)]
struct ComplexVariable {
    // ...
}

let mut complex_variable = ComplexVariable {
    // ...
};
let _result =  diff_ba::dbg!(&complex_variable, {
    // OPERATION YOU WANT TO VERIFY
    // The diff_ba::dbg! macro returns the final expression in this block.
});
```

## Very easy sample

```rust
use diff_ba_rs::prelude::*
let mut a = 2;
let b =  diff_ba::dbg!(&a,
    a *= 2;
    a + 1
});
// prints:
// ```
// - 2
// + 4
// ```
assert_eq!(b, 5);
```

## Note

For now, only the dbg! macro prints to standard output, but if there is demand for it from myself or others, I may implement a macro that returns a string or writes to a file.

## License

diff-ba-rs is released under the MIT License
