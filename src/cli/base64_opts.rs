use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    /// Encode or decode base64 strings
    #[command(name = "encode", about = "Encode or decode base64 strings")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Encode or decode base64 strings")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// The string to encode
    #[arg(short, long,default_value="-",value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long,value_parser=parse_base64_format, default_value ="Standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// The string to decode
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long,value_parser=parse_base64_format, default_value ="Standard")]
    pub format: Base64Format,
}
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format, anyhow::Error> {
    s.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Standard" => Ok(Base64Format::Standard),
            "Urlsafe" => Ok(Base64Format::Urlsafe),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}
impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "Standard",
            Base64Format::Urlsafe => "Urlsafe",
        }
    }
}
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "Standard"),
            Base64Format::Urlsafe => write!(f, "Urlsafe"),
        }
    }
}
