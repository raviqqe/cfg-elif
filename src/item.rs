pub use crate::{item_cfg as cfg, item_feature as feature};

/// Compiles expressions conditionally on features.
///
/// # Examples
///
/// ```rust
/// use cfg_exif::item::feature;
///
/// feature!(if ("foo") {
///     type Foo = usize;
/// } else if ("bar") {
///     type Foo = isize;
/// } else {
///     type Foo = f64;
/// });
/// ```
#[macro_export]
macro_rules! item_feature {
    (if ($name:literal) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {
        #[cfg(feature = $name)]
        $then1
        #[cfg(not(feature = $name))]
        $crate::item_feature!($(if $condition { $then2 } else)* { $else })
    };
    (if (!$name:literal) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {
        #[cfg(not(feature = $name))]
        $then1
        #[cfg(feature = $name)]
        $crate::item_feature!($(if $condition { $then2 } else)* { $else })
    };
    ({ $else:item }) => {{
        $else
    }};
}

/// Compiles expressions conditionally on compile configurations.
///
/// # Examples
///
/// ```rust
/// use cfg_exif::item::cfg;
///
/// cfg!(if (feature == "foo") {
///     type Foo = usize;
/// } else if (target_pointer_width != "64") {
///     type Foo = isize;
/// } else if ((target_family == "unix") && (feature == "bar")) {
///     type Foo = i32;
/// } else if ((feature == "baz") || (target_os == "freebsd")) {
///     type Foo = i64;
/// } else if (!(panic == "unwind")) {
///     type Foo = i128;
/// } else {
///     type Foo = f64;
/// });
///
/// assert_eq!(3.14 as Foo, 3.14);
/// ```
#[macro_export]
macro_rules! item_cfg {
    (if ($key:ident == $value:literal) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {
        #[cfg($key = $value)]
        $then1
        #[cfg(not($key = $value))]
        $crate::item_cfg!($(if $condition { $then2 } else)* { $else });
    };
    (if ($key:ident != $value:literal) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {
        #[cfg(not($key = $value))]
        $then1
        #[cfg($key = $value)]
        $crate::item_cfg!($(if $condition { $then2 } else)* { $else });
    };
    (if ($left:tt && $right:tt) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {
        $crate::item_cfg!(if $left {
            $crate::item_cfg!(if $right {
                $then1
            } else {
                $crate::item_cfg!($(if $condition { $then2 } else)* { $else });
            });
        } else {
            $crate::item_cfg!($(if $condition { $then2 } else)* { $else });
        });
    };
    (if ($left:tt || $right:tt) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {
        $crate::item_cfg!(if $left {
            $then1
        } else if $right {
            $then1
        } else {
            $crate::item_cfg!($(if $condition { $then2 } else)* { $else });
        });
    };
    (if (!$condition1:tt) { $then1:item } else $(if $condition2:tt { $then2:item } else)* { $else:item }) => {
        $crate::item_cfg!(if $condition1 {
            $crate::item_cfg!($(if $condition2 { $then2 } else)* { $else });
        } else {
            $then1
        });
    };
    ({ $else:item }) => {
        $else
    };
}
