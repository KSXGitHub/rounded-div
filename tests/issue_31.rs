macro_rules! div_min_by {
    ($dividend_type:ident, $divisor:expr) => {
        rounded_div::$dividend_type($dividend_type::MIN, $divisor)
    };
}

/// Link: <https://github.com/KSXGitHub/rounded-div/issues/31>
#[test]
fn issue_31() {
    assert_eq!(div_min_by!(i8, 1), i8::MIN);
    assert_eq!(div_min_by!(i8, 2), i8::MIN / 2);
    assert_eq!(div_min_by!(i8, 3), i8::MIN / 3 - 1);
}
