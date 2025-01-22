# cfg-elif

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/cfg-elif/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/cfg-elif/actions)
[![Crate](https://img.shields.io/crates/v/cfg-elif.svg?style=flat-square)](https://crates.io/crates/cfg-elif)
[![License](https://img.shields.io/github/license/raviqqe/cfg-elif.svg?style=flat-square)](UNLICENSE)

The `rustfmt`-friendly conditional compilation like [`cfg-if`](https://github.com/rust-lang/cfg-if).

## Features

- Conditional compilation at both expression and item positions
- `rustfmt` friendly

## Examples

```rust
use cfg_elif::{expr, item};

item::cfg!(if (feature == "foo") {
    type Foo = usize;
} else if (target_pointer_width != "64") {
    type Foo = isize;
} else if ((target_family == "unix") && (feature == "bar")) {
    type Foo = i32;
} else if ((feature == "baz") || (target_os == "freebsd")) {
    type Foo = i64;
} else if (!(panic == "unwind")) {
    type Foo = i128;
} else {
    type Foo = f64;
});

assert_eq!(3.14 as Foo, 3.14);

assert_eq!(
    expr::cfg!(if (feature == "foo") {
        0
    } else if (target_pointer_width != "64") {
        1
    } else if ((target_family == "unix") && (feature == "bar")) {
        2
    } else if ((feature == "baz") || (target_os == "freebsd")) {
        3
    } else if (!(panic == "unwind")) {
        4
    } else {
        42
    }),
    42
);
```

## License

[The Unlicense](UNLICENSE)
