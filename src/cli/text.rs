use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    /// Encode or decode base64 strings
    #[command(name = "sign", about = "")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "")]
    Verify(TextVerifyOpts),
}
#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// The string to encode
    #[arg(short, long,default_value="-",value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(long,value_parser=parse_text_format, default_value ="Blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// The string to encode
    #[arg(short, long,default_value="-",value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long,default_value="-",value_parser=verify_input_file)]
    pub sign: String,
    #[arg(long,value_parser=parse_text_format, default_value ="Blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_format(s: &str) -> Result<TextSignFormat, anyhow::Error> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Blake3" => Ok(TextSignFormat::Blake3),
            "Ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "Blake3",
            TextSignFormat::Ed25519 => "Ed25519",
        }
    }
}
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "Blake3"),
            TextSignFormat::Ed25519 => write!(f, "Ed25519"),
        }
    }
}
