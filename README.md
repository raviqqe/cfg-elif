# cfg-exif

[![GitHub Action](https://img.shields.io/github/actions/workflow/status/raviqqe/cfg-exif/test.yaml?branch=main&style=flat-square)](https://github.com/raviqqe/cfg-exif/actions)
[![Crate](https://img.shields.io/crates/v/cfg-exif.svg?style=flat-square)](https://crates.io/crates/cfg-exif)
[![License](https://img.shields.io/github/license/raviqqe/cfg-exif.svg?style=flat-square)](UNLICENSE)

The `rustfmt`-friendly [`cfg-if`](https://github.com/rust-lang/cfg-if).

## Features

- Conditional compilation at both expression and item positions
- `rustfmt` friendly

## Examples

```rust
use cfg_exif::{cfg, item};

item::cfg!(if (feature == "foo") {
    type Foo = usize;
} else if (target_os == "linux") {
    type Foo = isize;
} else {
    type Foo = f64;
});

assert_eq!(
    cfg!(if (feature == "foo") {
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
