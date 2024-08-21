#![doc = include_str!("../README.md")]
#![no_std]

/// Compiles expressions conditionally on features.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///     cfg_exif::feature!(if ("foo") {
///         0
///     } else if (!"bar") {
///         42
///     } else {
///         1
///     }),
///     42
/// );
/// ```
#[macro_export]
macro_rules! feature {
    (if ($name:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {
        {
            #[cfg(feature = $name)]
            { $then1 }
            #[cfg(not(feature = $name))]
            { $crate::feature!($(if $condition { $then2 } else)* { $else }) }
        }
    };
    (if (!$name:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {
        {
            #[cfg(not(feature = $name))]
            { $then1 }
            #[cfg(feature = $name)]
            { $crate::feature!($(if $condition { $then2 } else)* { $else }) }
        }
    };
    ({ $else:expr }) => {{
        {
            $else
        }
    }};
}

/// Compiles expressions conditionally on compile configurations.
///
/// # Examples
///
/// ```rust
/// # use cfg_exif::cfg;
/// # fn main() {
/// assert_eq!(
///     cfg!(if (feature == "foo") {
///         0
///     } else if (target_os != "fuchsia") {
///         42
///     } else {
///         1
///     }),
///     42
/// );
/// # }
/// ```
#[macro_export]
macro_rules! cfg {
    (if ($key:ident == $value:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        #[cfg($key = $value)]
        {
            $then1
        }
        #[cfg(not($key = $value))]
        {
            $crate::cfg!($(if $condition { $then2 } else)* { $else })
        }
    }};
    (if ($key:ident != $value:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        #[cfg(not($key = $value))]
        {
            $then1
        }
        #[cfg($key = $value)]
        {
            $crate::cfg!($(if $condition { $then2 } else)* { $else })
        }
    }};
    ({ $else:expr }) => {{
        {
            $else
        }
    }};
}
