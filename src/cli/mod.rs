mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http;
mod text;

use std::path::Path;

use crate::{process_csv, process_genpass, CmdExecutor};

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
impl CmdExecutor for SubCommand {
    async fn exec(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => {
                let output: String = if let Some(output) = &opts.output {
                    output.clone()
                } else {
                    format!("output.{}", opts.format)
                };
                process_csv(&opts.input, output, opts.format)?;
            }
            SubCommand::GenPass(opts) => {
                process_genpass(opts)?;
            }
            SubCommand::Base64(opts) => opts.exec().await?,
            SubCommand::Text(opts) => opts.exec().await?,
            SubCommand::Http(opts) => opts.exec().await?,
        }
        Ok(())
    }
}
fn verify_input_file(s: &str) -> Result<String, &'static str> {
    if s == "-" || Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("File {} does not exist")
    }
}
