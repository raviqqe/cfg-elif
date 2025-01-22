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
/// } else if (target_os == "linux") {
///     type Foo = isize;
/// } else {
///     type Foo = f64;
/// });
/// ```
#[macro_export]
macro_rules! item_cfg {
    (if ($key:ident == $value:literal) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {{
        #[cfg($key = $value)]
        $then1
        #[cfg(not($key = $value))]
        $crate::item_cfg!($(if $condition { $then2 } else)* { $else })
    }};
    (if ($key:ident != $value:literal) { $then1:item } else $(if $condition:tt { $then2:item } else)* { $else:item }) => {{
        #[cfg(not($key = $value))]
        $then1
        #[cfg($key = $value)]
        $crate::item_cfg!($(if $condition { $then2 } else)* { $else })
    }};
    ({ $else:item }) => {{
        $else
    }};
}
