use rounded_div::RoundedDiv;

macro_rules! single_case {
    ($name:ident -> $dividend:literal / $divisor:literal == $expected:literal) => {
        let dividend: $name = $dividend;
        let divisor: $name = $divisor;
        let expected: $name = $expected;
        eprintln!("expect: {:>3} / {:>2} = {:>3}", dividend, divisor, expected);
        let actual: $name = dividend.rounded_div(divisor);
        assert_eq!(actual, expected);
    };
}

macro_rules! test_unsigned {
    ($name:ident) => {
        #[test]
        fn $name() {
            single_case!($name -> 59 / 4 == 15);
            single_case!($name -> 58 / 4 == 15);
            single_case!($name -> 57 / 4 == 14);
            single_case!($name ->  0 / 4 ==  0);
        }
    };
}

macro_rules! test_signed {
    ($name:ident) => {
        #[test]
        fn $name() {
            single_case!($name ->  59 /  4 ==  15);
            single_case!($name ->  58 /  4 ==  15);
            single_case!($name ->  57 /  4 ==  14);
            single_case!($name -> -59 /  4 == -15);
            single_case!($name -> -58 /  4 == -15);
            single_case!($name -> -57 /  4 == -14);
            single_case!($name ->  59 / -4 == -15);
            single_case!($name ->  58 / -4 == -15);
            single_case!($name ->  57 / -4 == -14);
            single_case!($name -> -59 / -4 ==  15);
            single_case!($name -> -58 / -4 ==  15);
            single_case!($name -> -57 / -4 ==  14);
            single_case!($name ->   0 /  4 ==   0);
            single_case!($name ->   0 / -4 ==   0);
        }
    };
}

test_unsigned!(u8);
test_unsigned!(u16);
test_unsigned!(u32);
test_unsigned!(u64);
test_unsigned!(u128);
test_unsigned!(usize);

test_signed!(i8);
test_signed!(i16);
test_signed!(i32);
test_signed!(i64);
test_signed!(i128);
test_signed!(isize);
