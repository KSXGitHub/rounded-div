use structopt::StructOpt;

/// The CLI arguments.
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Get rounded result of an integer division",
    usage = "rounded-div <SUBCOMMAND> <dividend> <divisor>",
    rename_all = "kebab-case"
)]
pub enum Args {
    #[structopt(about = "Perform rounded division on two u8 numbers")]
    U8 { dividend: u8, divisor: u8 },
    #[structopt(about = "Perform rounded division on two u16 numbers")]
    U16 { dividend: u16, divisor: u16 },
    #[structopt(about = "Perform rounded division on two u32 numbers")]
    U32 { dividend: u32, divisor: u32 },
    #[structopt(about = "Perform rounded division on two u64 numbers")]
    U64 { dividend: u64, divisor: u64 },
    #[structopt(about = "Perform rounded division on two u128 numbers")]
    U128 { dividend: u128, divisor: u128 },
    #[structopt(about = "Perform rounded division on two usize numbers")]
    Usize { dividend: usize, divisor: usize },
    #[structopt(about = "Perform rounded division on two i8 numbers")]
    I8 { dividend: i8, divisor: i8 },
    #[structopt(about = "Perform rounded division on two i16 numbers")]
    I16 { dividend: i16, divisor: i16 },
    #[structopt(about = "Perform rounded division on two i32 numbers")]
    I32 { dividend: i32, divisor: i32 },
    #[structopt(about = "Perform rounded division on two i64 numbers")]
    I64 { dividend: i64, divisor: i64 },
    #[structopt(about = "Perform rounded division on two i128 numbers")]
    I128 { dividend: i128, divisor: i128 },
    #[structopt(about = "Perform rounded division on two isize numbers")]
    Isize { dividend: isize, divisor: isize },
}
