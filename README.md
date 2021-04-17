# rounded-div

[![Test](https://github.com/KSXGitHub/rounded-div/workflows/Test/badge.svg)](https://github.com/KSXGitHub/rounded-div/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/rounded-div?logo=rust)](https://crates.io/crates/rounded-div)

Get rounded result of an integer division.

## Usage Examples

### `const fn rounded_div_*`

```rust
assert_eq!(rounded_div::i32( 59, 4),  15); // 59/4 is equal to 14.75 which is closer to 15
assert_eq!(rounded_div::i32( 58, 4),  15); // 59/4 is equal to 14.5 which is rounded to 15
assert_eq!(rounded_div::i32( 57, 4),  14); // 59/4 is equal to 14.25 which is closer to 14
assert_eq!(rounded_div::i32(-59, 4), -15);
assert_eq!(rounded_div::i32(-58, 4), -15);
assert_eq!(rounded_div::i32(-57, 4), -14);
```

### `trait RoundedDiv`

```rust
use rounded_div::RoundedDiv;
assert_eq!( 59i32.rounded_div(4),  15); // 59/4 is equal to 14.75 which is closer to 15
assert_eq!( 58i32.rounded_div(4),  15); // 59/4 is equal to 14.5 which is rounded to 15
assert_eq!( 57i32.rounded_div(4),  14); // 59/4 is equal to 14.25 which is closer to 14
assert_eq!(-59i32.rounded_div(4), -15);
assert_eq!(-58i32.rounded_div(4), -15);
assert_eq!(-57i32.rounded_div(4), -14);
```

## License

[MIT](https://git.io/JO4Rf) © [Hoàng Văn Khải](https://ksxgithub.github.io/).
