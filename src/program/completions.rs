use super::Args;
use structopt_utilities::StructOptUtils;

/// The program that generate completion files for the main program.
pub fn main() {
    Args::run_completion_generator("rounded-div-completions", "rounded-div");
}
