use clap::Parser;
use std::{fmt, path::Path, str::FromStr};
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_file_exists)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, value_parser= parse_format,default_value = "json")]
    pub format: OutputFormat,
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
fn parse_format(s: &str) -> Result<OutputFormat, anyhow::Error> {
    s.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(f: OutputFormat) -> &'static str {
        match f {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
