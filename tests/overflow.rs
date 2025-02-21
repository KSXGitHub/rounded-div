macro_rules! test_case {
    ($name:ident) => {
        #[test]
        #[should_panic(expected = "attempt to divide with overflow")]
        fn $name() {
            let _ = rounded_div::$name($name::MIN, -1);
        }
    };
}

test_case!(i8);
test_case!(i16);
test_case!(i32);
test_case!(i64);
test_case!(i128);
