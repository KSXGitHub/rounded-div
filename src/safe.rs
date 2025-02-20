//! Safe rounded div forma with protection against integer overflow.

/// Get rounded result of an integer division.
pub trait RoundedDiv<Rhs = Self> {
    /// Type of the rounded result.
    type Output;
    /// Get rounded result of an integer division.
    fn rounded_div(self, rhs: Rhs) -> Self::Output;
}

macro_rules! div_fn {
    ($name:ident -> $data:ident do as $resize:ident) => {
        /// Get rounded result of an integer division.
        pub const fn $name(dividend: $data, divisor: $data) -> $data {
            let dividend = dividend as $resize;
            let divisor = divisor as $resize;
            crate::naive::$resize(dividend, divisor) as $data
        }

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

div_fn!(rounded_div_u8 -> u8 do as u16);
div_fn!(rounded_div_u16 -> u16 do as u32);
div_fn!(rounded_div_u32 -> u32 do as u64);
div_fn!(rounded_div_u64 -> u64 do as u128);
div_fn!(rounded_div_usize -> usize do as u128);

div_fn!(rounded_div_i8 -> i8 do as i16);
div_fn!(rounded_div_i16 -> i16 do as i32);
div_fn!(rounded_div_i32 -> i32 do as i64);
div_fn!(rounded_div_i64 -> i64 do as i128);
div_fn!(rounded_div_isize -> isize do as i128);
