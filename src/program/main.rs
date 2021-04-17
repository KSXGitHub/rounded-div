use super::Args::{self, *};
use crate::{i128, i16, i32, i64, i8, isize, u128, u16, u32, u64, u8, usize};
use structopt_utilities::StructOptUtils;

/// The main program.
pub fn main() {
    macro_rules! run {
        ($func:ident $dividend:ident $divisor:ident) => {
            println!("{}", $func($dividend, $divisor));
        };
    }

    match Args::strict_from_args() {
        U8 { dividend, divisor } => run!(u8 dividend divisor),
        U16 { dividend, divisor } => run!(u16 dividend divisor),
        U32 { dividend, divisor } => run!(u32 dividend divisor),
        U64 { dividend, divisor } => run!(u64 dividend divisor),
        U128 { dividend, divisor } => run!(u128 dividend divisor),
        Usize { dividend, divisor } => run!(usize dividend divisor),
        I8 { dividend, divisor } => run!(i8 dividend divisor),
        I16 { dividend, divisor } => run!(i16 dividend divisor),
        I32 { dividend, divisor } => run!(i32 dividend divisor),
        I64 { dividend, divisor } => run!(i64 dividend divisor),
        I128 { dividend, divisor } => run!(i128 dividend divisor),
        Isize { dividend, divisor } => run!(isize dividend divisor),
    }
}
