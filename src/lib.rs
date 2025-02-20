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

pub mod naive;
pub mod safe;

pub use naive::NaiveRoundedDiv;
pub use safe::*;
