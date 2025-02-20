//! Raw, naive rounded division formula without protection against integer overflow.

/// Get rounded result of an integer division without integer overflow protection.
pub trait NaiveRoundedDiv<Rhs = Self> {
    /// Type of the rounded result.
    type Output;
    /// Get rounded result of an integer division without integer overflow protection.
    fn naive_rounded_div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! aliases {
    ($name:ident -> $data:ident) => {
        impl NaiveRoundedDiv for $data {
            type Output = Self;

            #[inline]
            fn naive_rounded_div(self, rhs: $data) -> Self::Output {
                $name(self, rhs)
            }
        }

        pub use $name as $data;
    };
}

macro_rules! unsigned_div_fn {
    ($name:ident -> $data:ident) => {
        /// Get rounded result of an integer division without integer overflow protection.
        #[inline]
        pub const fn $name(dividend: $data, divisor: $data) -> $data {
            (dividend + (divisor >> 1)) / divisor
        }

        aliases!($name -> $data);
    };
}

macro_rules! signed_div_fn {
    ($name:ident -> $data:ident) => {
        /// Get rounded result of an integer division without integer overflow protection.
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

unsigned_div_fn!(naive_rounded_div_u8 -> u8);
unsigned_div_fn!(naive_rounded_div_u16 -> u16);
unsigned_div_fn!(naive_rounded_div_u32 -> u32);
unsigned_div_fn!(naive_rounded_div_u64 -> u64);
unsigned_div_fn!(naive_rounded_div_u128 -> u128);
unsigned_div_fn!(naive_rounded_div_usize -> usize);

signed_div_fn!(naive_rounded_div_i8 -> i8);
signed_div_fn!(naive_rounded_div_i16 -> i16);
signed_div_fn!(naive_rounded_div_i32 -> i32);
signed_div_fn!(naive_rounded_div_i64 -> i64);
signed_div_fn!(naive_rounded_div_i128 -> i128);
signed_div_fn!(naive_rounded_div_isize -> isize);
