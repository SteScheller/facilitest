//pub use paste::paste;

use facilitest_macros::{build_ident, make_test_fn};

/*
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
*/

#[macro_export]
macro_rules! test_p {
    ($func:path, ($($suffix:ident: ($($arg:expr),*), $expected:expr)*)) => {
    $(
        make_test_fn!($func, $suffix, ($($arg,)*), $expected);
    )*
    }
}
