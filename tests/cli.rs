use command_extra::CommandExtra;
use pipe_trait::Pipe;
use std::{
    fmt::Debug,
    process::{Command, Stdio},
    str::FromStr,
};

/// Calculate rounded division by executing the `rounded-div` program.
fn execute<Input, Output>(function: &'static str, dividend: Input, divisor: Input) -> Output
where
    Input: ToString + Debug,
    Output: FromStr,
    Output::Err: Debug,
{
    let output = env!("CARGO_BIN_EXE_rounded-div")
        .pipe(Command::new)
        .with_arg(function)
        .with_arg("--")
        .with_arg(dividend.to_string())
        .with_arg(divisor.to_string())
        .with_stdin(Stdio::null())
        .with_stdout(Stdio::piped())
        .with_stderr(Stdio::inherit())
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "rounded-div {function} {dividend:?} {divisor:?}: {error}",
                function = function,
                dividend = &dividend,
                divisor = &divisor,
                error = error,
            )
        });

    assert!(output.status.success());
    output
        .stdout
        .pipe(String::from_utf8)
        .unwrap()
        .trim_end()
        .parse()
        .unwrap()
}

macro_rules! single_case {
    ($name:ident -> $dividend:literal / $divisor:literal == $expected:literal) => {
        let dividend: $name = $dividend;
        let divisor: $name = $divisor;
        let expected: $name = $expected;
        eprintln!("expect: {:>3} / {:>2} = {:>3}", dividend, divisor, expected);
        let actual: $name = execute(stringify!($name), dividend, divisor);
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
