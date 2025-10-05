#![no_std]
//! Get rounded result of an integer division.
//!
//! ### `const fn rounded_div_*`
//!
//! ```
//! assert_eq!(rounded_div::i32( 59, 4),  15); // 59/4 is equal to 14.75 which is closer to 15
//! assert_eq!(rounded_div::i32( 58, 4),  15); // 58/4 is equal to 14.5 which is rounded to 15
//! assert_eq!(rounded_div::i32( 57, 4),  14); // 57/4 is equal to 14.25 which is closer to 14
//! assert_eq!(rounded_div::i32(-59, 4), -15);
//! assert_eq!(rounded_div::i32(-58, 4), -15);
//! assert_eq!(rounded_div::i32(-57, 4), -14);
//! ```
//!
//! ### `trait RoundedDiv`
//!
//! ```
//! use rounded_div::RoundedDiv;
//! assert_eq!( 59i32.rounded_div(4),  15); // 59/4 is equal to 14.75 which is closer to 15
//! assert_eq!( 58i32.rounded_div(4),  15); // 58/4 is equal to 14.5 which is rounded to 15
//! assert_eq!( 57i32.rounded_div(4),  14); // 57/4 is equal to 14.25 which is closer to 14
//! assert_eq!(-59i32.rounded_div(4), -15);
//! assert_eq!(-58i32.rounded_div(4), -15);
//! assert_eq!(-57i32.rounded_div(4), -14);
//! ```

/// Get rounded result of an integer division.
pub trait RoundedDiv<Rhs = Self> {
    /// Type of the rounded result.
    type Output;
    /// Get rounded result of an integer division.
    fn rounded_div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! aliases {
    ($name:ident -> $data:ident) => {
        impl RoundedDiv for $data {
            type Output = Self;

            #[inline]
            fn rounded_div(self, rhs: $data) -> Self::Output {
                $name(self, rhs)
            }
        }

        pub use $name as $data;
    };
}

macro_rules! unsigned_div_fn {
    ($name:ident -> $data:ident) => {
        /// Get rounded result of an integer division.
        #[inline]
        pub const fn $name(dividend: $data, divisor: $data) -> $data {
            let quotient = dividend / divisor;
            let remainder = dividend % divisor;
            quotient + (remainder << 1 >= divisor) as $data
        }

        aliases!($name -> $data);
    };
}

macro_rules! signed_div_fn {
    ($name:ident -> $data:ident do as $unsigned:ident) => {
        /// Get rounded result of an integer division.
        pub const fn $name(dividend: $data, divisor: $data) -> $data {
            // Q: No negative sign before `$data::MIN` to invert it. Is it correct?
            // A: It is correct and defined.
            // Q: Why do it? Why don't we just use `-`?
            // A: Because `-$data::MIN` would panic with overflow in debug mode.
            const NEG_OF_MIN: $unsigned = $data::MIN as $unsigned;

            // The following constants are necessary because older version of Rust does not
            // support constant expressions in match arms.
            const HALF_MIN: $data = $data::MIN / 2;
            const HALF_MIN_PLUS_1: $data = HALF_MIN + 1;
            const NEG_HALF_MIN: $data = -HALF_MIN;
            const NEG_HALF_MIN_MINUS_1: $data = NEG_HALF_MIN - 1;
            const HALF_MAX: $data = $data::MAX / 2;
            const HALF_MAX_PLUS_1: $data = HALF_MAX + 1;
            const NEG_HALF_MAX: $data = -HALF_MAX;
            const NEG_HALF_MAX_MINUS_1: $data = NEG_HALF_MAX - 1;

            match (dividend, divisor) {
                ($data::MIN, -1) => panic!("attempt to divide with overflow"),
                ($data::MIN, $data::MIN) | ($data::MAX, $data::MAX) => 1,
                ($data::MIN, $data::MAX) | ($data::MAX, $data::MIN) => -1,

                (0, ..=-1 | 1..) => 0,
                (..=-1 | 1.., 1) => dividend,
                (..=-1 | 1.., -1) => -dividend,

                (_, 0) => panic!("attempt to divide by zero"),

                ($data::MIN, 1..) => -(self::$unsigned(NEG_OF_MIN, divisor as $unsigned) as $data),
                ($data::MIN, ..=-1) => self::$unsigned(NEG_OF_MIN, -divisor as $unsigned) as $data,

                (HALF_MIN_PLUS_1..=NEG_HALF_MIN_MINUS_1, $data::MIN) => 0,// MIN is even, so x/MIN where x.abs() < MIN/2 should be less than 0.5
                (..=HALF_MIN, $data::MIN) => 1,// MIN is even so x/MIN where x <= MIN/2 should be greater than 0.5
                (NEG_HALF_MIN.., $data::MIN) => -1, // MIN is even so x/MIN where x >= -MIN/2 should be less than -0.5

                (NEG_HALF_MAX..=HALF_MAX, $data::MAX) => 0, // MAX is odd, so x/MAX where x.abs() <= MAX/2 should be less than 0.5
                (HALF_MAX_PLUS_1.., $data::MAX) => 1, // MAX is odd so x/MAX where x > MAX/2 should be greater than 0.5
                (..=NEG_HALF_MAX_MINUS_1, $data::MAX) => -1, // MAX is odd so x/MAX where x < -MAX/2 should be less than -0.5

                (1.., 1..) => self::$unsigned(dividend as $unsigned, divisor as $unsigned) as $data,
                (..=-1, 1..) => -(self::$unsigned(-dividend as $unsigned, divisor as $unsigned) as $data),
                (1.., ..=-1) => -(self::$unsigned(dividend as $unsigned, -divisor as $unsigned) as $data),
                (..=-1, ..=-1) => self::$unsigned(-dividend as $unsigned, -divisor as $unsigned) as $data,
            }
        }

        aliases!($name -> $data);
    };
}

unsigned_div_fn!(rounded_div_u8 -> u8);
unsigned_div_fn!(rounded_div_u16 -> u16);
unsigned_div_fn!(rounded_div_u32 -> u32);
unsigned_div_fn!(rounded_div_u64 -> u64);
unsigned_div_fn!(rounded_div_u128 -> u128);
unsigned_div_fn!(rounded_div_usize -> usize);

signed_div_fn!(rounded_div_i8 -> i8 do as u8);
signed_div_fn!(rounded_div_i16 -> i16 do as u16);
signed_div_fn!(rounded_div_i32 -> i32 do as u32);
signed_div_fn!(rounded_div_i64 -> i64 do as u64);
signed_div_fn!(rounded_div_i128 -> i128 do as u128);
signed_div_fn!(rounded_div_isize -> isize do as usize);
