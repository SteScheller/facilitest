//pub use paste::paste;

pub use facilitest_macros::cat_ident;


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
    ($func:ident, ($($suffix:ident: ($($arg:expr),*), $expected:expr)*)) => {
    $(
        //#[test]
        fn cat_ident!(test_, $func, $suffix)() {
        //fn func() {
            //cat_ident!(test_, $func, $suffix)();
            assert_eq!($func($($arg,)*), $expected);
        }
    )*
    }
}
