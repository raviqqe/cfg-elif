use cfg_if::cfg;

type Foo = cfg!(if (feature == "foo") {
    usize
} else if (feature == "bar") {
    isize
} else {
    f64
});
