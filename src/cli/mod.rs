mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http;
mod text;

use std::path::Path;

use self::csv_opts::CsvOpts;
pub use self::{
    base64_opts::Base64Format, base64_opts::Base64Subcommand, csv_opts::OutputFormat,
    genpass_opts::GenPassOpts, http::HttpSubcommand, text::TextSignFormat, text::TextSubcommand,
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
    #[command(subcommand)]
    Text(TextSubcommand),
    #[command(subcommand)]
    Http(HttpSubcommand),
}
fn verify_input_file(s: &str) -> Result<String, &'static str> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("File {} does not exist")
    }
}
