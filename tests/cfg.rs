use cfg_exif::expr::cfg;

#[test]
fn feature() {
    assert_eq!(cfg!(if (feature == "foo") { 0 } else { 42 }), 42);
    assert_eq!(
        cfg!(if (feature == "foo") {
            0
        } else if (feature == "bar") {
            1
        } else {
            42
        }),
        42
    );
}

#[test]
fn target_os() {
    assert_eq!(cfg!(if (target_os == "windows") { 0 } else { 42 }), 42);
    assert_eq!(
        cfg!(if (target_os == "windows") {
            0
        } else if (target_os == "fuchsia") {
            1
        } else {
            42
        }),
        42
    );
}

#[test]
fn not() {
    assert_eq!(cfg!(if (feature != "foo") { 42 } else { 0 }), 42);
    assert_eq!(
        cfg!(if (feature != "foo") {
            42
        } else if (target_os != "fuchsia") {
            0
        } else {
            1
        }),
        42
    );
}

#[test]
fn mix() {
    assert_eq!(
        cfg!(if (feature == "foo") {
            0
        } else if (target_os != "fuchsia") {
            42
        } else {
            1
        }),
        42
    );
    assert_eq!(
        cfg!(if (target_os != "fuchsia") {
            42
        } else if (feature == "foo") {
            0
        } else {
            1
        }),
        42
    );
}
