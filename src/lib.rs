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
        #[doc = "Get rounded result of an integer division."]
        #[inline]
        pub const fn $name(dividend: $data, divisor: $data) -> $data {
            (dividend + (divisor >> 1)) / divisor
        }

        aliases!($name -> $data);
    };
}

macro_rules! signed_div_fn {
    ($name:ident -> $data:ident) => {
        #[doc = "Get rounded result of an integer division."]
        #[inline]
        pub const fn $name(dividend: $data, divisor: $data) -> $data {
            if dividend ^ divisor >= 0 {
                (dividend + (divisor / 2)) / divisor
            } else {
                (dividend - (divisor / 2)) / divisor
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

signed_div_fn!(rounded_div_i8 -> i8);
signed_div_fn!(rounded_div_i16 -> i16);
signed_div_fn!(rounded_div_i32 -> i32);
signed_div_fn!(rounded_div_i64 -> i64);
signed_div_fn!(rounded_div_i128 -> i128);
signed_div_fn!(rounded_div_isize -> isize);
