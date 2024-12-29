mod cli;
mod process;

pub use cli::{Base64Format, Base64Subcommand, GenPassOpts, Opts, SubCommand};
pub use process::process_base64_decode;
pub use process::process_base64_encode;
pub use process::process_csv;
pub use process::process_genpass;
