use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    /// Encode or decode base64 strings
    #[command(about = "")]
    Serve(HttpServeOpts),
}
#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
