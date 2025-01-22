pub use crate::{parens_cfg as cfg, parens_feature as feature};

/// Compiles expressions conditionally on features.
///
/// # Examples
///
/// ```rust
/// use cfg_exif::parens::feature;
///
/// type Foo = feature!(if ("foo") { usize } else { f64 });
///
/// assert_eq!(
///     feature!(if ("foo") {
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
macro_rules! parens_feature {
    (if ($name:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {
        #[cfg(feature = $name)]
        ($then1)
        #[cfg(not(feature = $name))]
        (feature!($(if $condition { $then2 } else)* { $else }))
    };
    (if (!$name:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {
        #[cfg(not(feature = $name))]
        ($then1)
        #[cfg(feature = $name)]
        ($crate::feature!($(if $condition { $then2 } else)* { $else }))
    };
    ({ $else:expr }) => {{
        $else
    }};
}

/// Compiles expressions conditionally on compile configurations.
///
/// # Examples
///
/// ```rust
/// use cfg_exif::cfg;
///
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
/// ```
#[macro_export]
macro_rules! parens_cfg {
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
