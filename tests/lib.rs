#[cfg(test)]
mod tests {
    use facilitest::*;

    use std::convert::identity;

    /*
    test_suite! {
        foo: (identity, 42, 42),
        bar: (identity, 0xC0FFEE, 0xC0FFEE),
    }
    */

    test_p! {
        identity,
        (
            _0: ("D'oh!"), "D'oh!"
            _1: (4711), 4711
        )
    }

    test_p! {
        std::cmp::max,
        (
            _0: (-10, -1), -1
            _1: (-10, 10), 10
            _2: ("a", "A"), "a"
        )
    }

    #[test]
    fn test_build_ident() {
        let foobar: Option<i32> = None;
        assert_eq!(&build_ident!(foo, bar), &foobar);
    }
}
