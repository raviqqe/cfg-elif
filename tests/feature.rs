use cfg_elif::expr::feature;

#[test]
fn feature() {
    assert_eq!(feature!(if ("foo") { 13 } else { 42 }), 42);
    assert_eq!(
        feature!(if ("foo") {
            13
        } else if ("bar") {
            13
        } else {
            42
        }),
        42
    );
    assert_eq!(
        feature!(if ("foo") {
            13
        } else if ("bar") {
            13
        } else if ("baz") {
            13
        } else {
            42
        }),
        42
    );
}

#[test]
fn not() {
    assert_eq!(feature!(if (!"foo") { 42 } else { 13 }), 42);
}

#[test]
fn mix() {
    assert_eq!(
        feature!(if (!"foo") {
            42
        } else if ("bar") {
            13
        } else {
            13
        }),
        42
    );
    assert_eq!(
        feature!(if ("foo") {
            13
        } else if (!"bar") {
            42
        } else {
            13
        }),
        42
    );
}
