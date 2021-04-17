use structopt::StructOpt;

/// The CLI arguments.
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Get rounded result of an integer division",
    usage = "rounded-div <SUBCOMMAND> [dividend] [divisor]",
    rename_all = "kebab-case"
)]
pub enum Args {
    #[structopt(about = "Perform rounded division on two unsigned 8-bit integers")]
    U8 { dividend: u8, divisor: u8 },
    #[structopt(about = "Perform rounded division on two unsigned 16-bit integers")]
    U16 { dividend: u16, divisor: u16 },
    #[structopt(about = "Perform rounded division on two unsigned 32-bit integers")]
    U32 { dividend: u32, divisor: u32 },
    #[structopt(about = "Perform rounded division on two unsigned 64-bit integers")]
    U64 { dividend: u64, divisor: u64 },
    #[structopt(about = "Perform rounded division on two unsigned 128-bit integers")]
    U128 { dividend: u128, divisor: u128 },
    #[structopt(about = "Perform rounded division on two pointer-sized unsigned integers")]
    Usize { dividend: usize, divisor: usize },
    #[structopt(about = "Perform rounded division on two signed 8-bit integers")]
    I8 { dividend: i8, divisor: i8 },
    #[structopt(about = "Perform rounded division on two signed 16-bit integers")]
    I16 { dividend: i16, divisor: i16 },
    #[structopt(about = "Perform rounded division on two signed 32-bit integers")]
    I32 { dividend: i32, divisor: i32 },
    #[structopt(about = "Perform rounded division on two signed 64-bit integers")]
    I64 { dividend: i64, divisor: i64 },
    #[structopt(about = "Perform rounded division on two signed 128-bit integers")]
    I128 { dividend: i128, divisor: i128 },
    #[structopt(about = "Perform rounded division on two pointer-sized signed integers")]
    Isize { dividend: isize, divisor: isize },
}
