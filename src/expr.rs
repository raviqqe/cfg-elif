//! Conditional compilation at expression positions.

pub use crate::{expr_cfg as cfg, expr_feature as feature};

/// Compiles expressions conditionally on features.
///
/// # Examples
///
/// ```rust
/// use cfg_exif::expr::feature;
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
macro_rules! expr_feature {
    (if ($name:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {
        {
            #[cfg(feature = $name)]
            { $then1 }
            #[cfg(not(feature = $name))]
            { $crate::expr_feature!($(if $condition { $then2 } else)* { $else }) }
        }
    };
    (if (!$name:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {
        {
            #[cfg(not(feature = $name))]
            { $then1 }
            #[cfg(feature = $name)]
            { $crate::expr_feature!($(if $condition { $then2 } else)* { $else }) }
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
/// use cfg_exif::expr::cfg;
///
/// assert_eq!(
///     cfg!(if (feature == "foo") {
///         0
///     } else if (target_pointer_width != "64") {
///         1
///     } else if ((target_family == "unix") && (feature == "bar")) {
///         2
///     } else if ((feature == "baz") || (target_os == "freebsd")) {
///         3
///     } else if (!(panic == "unwind")) {
///         4
///     } else {
///         42
///     }),
///     42
/// );
/// ```
#[macro_export]
macro_rules! expr_cfg {
    (if ($key:ident == $value:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        #[cfg($key = $value)]
        {
            $then1
        }
        #[cfg(not($key = $value))]
        {
            $crate::expr_cfg!($(if $condition { $then2 } else)* { $else })
        }
    }};
    (if ($key:ident != $value:literal) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        #[cfg(not($key = $value))]
        {
            $then1
        }
        #[cfg($key = $value)]
        {
            $crate::expr_cfg!($(if $condition { $then2 } else)* { $else })
        }
    }};
    (if ($left:tt && $right:tt) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        $crate::expr_cfg!(if $left {
            $crate::expr_cfg!(if $right {
                $then1
            } else {
                $crate::expr_cfg!($(if $condition { $then2 } else)* { $else })
            })
        } else {
            $crate::expr_cfg!($(if $condition { $then2 } else)* { $else })
        })
    }};
    (if ($left:tt || $right:tt) { $then1:expr } else $(if $condition:tt { $then2:expr } else)* { $else:expr }) => {{
        $crate::expr_cfg!(if $left {
            $then1
        } else if $right {
            $then1
        } else {
            $crate::expr_cfg!($(if $condition { $then2 } else)* { $else })
        })
    }};
    (if (!$condition1:tt) { $then1:expr } else $(if $condition2:tt { $then2:expr } else)* { $else:expr }) => {{
        $crate::expr_cfg!(if $condition1 {
            $crate::expr_cfg!($(if $condition2 { $then2 } else)* { $else })
        } else {
            $then1
        })
    }};
    ({ $else:expr }) => {{
        {
            $else
        }
    }};
}
