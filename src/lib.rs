pub use paste::paste;

#[macro_export]
macro_rules! test_suite {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (func, input, expected) = $value;
            assert_eq!(func(input), expected);
        }
    )*
    }
}

#[macro_export]
macro_rules! test_p {
    ($func:ident, ($($suffix:ident: ($($arg:expr),*), $expected:expr)*)) => {
    $(
        $crate::paste! {
            #[test]
            fn [<test_$func$suffix>]() {
                assert_eq!($func($($arg,)*), $expected);
            }
        }
    )*
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::identity;

    test_suite! {
        foo: (identity, 42, 42),
        bar: (identity, 0xC0FFEE, 0xC0FFEE),
    }

    test_p! {
        identity,
        (
            _0: ("D'oh!"), "D'oh!"
            _1: (4711), 4711
        )
    }
}
