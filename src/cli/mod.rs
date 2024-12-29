mod base64_opts;
mod csv_opts;
mod genpass_opts;

use std::path::Path;

use self::csv_opts::CsvOpts;
pub use self::{
    base64_opts::Base64Format, base64_opts::Base64Subcommand, csv_opts::OutputFormat,
    genpass_opts::GenPassOpts,
};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", author, version, about = "a rust cli tool")]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "csv to json")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Subcommand),
}
fn verify_input_file(s: &str) -> Result<String, &'static str> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("File {} does not exist")
    }
}
