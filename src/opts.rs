use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_file_exists)]
    pub input: String,
    #[arg(short, long, default_value = "output.jsonb")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(s: &str) -> Result<String, &'static str> {
    if Path::new(s).exists() {
        Ok(s.to_string())
    } else {
        Err("File {} does not exist")
    }
}
