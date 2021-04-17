//! Code for the CLI programs.

pub mod args;
pub mod completions;
pub mod main;

pub use args::Args;

pub use structopt;
pub use structopt::clap;
pub use structopt_utilities;
