use std::path::PathBuf;

use clap::Parser;

use crate::{process_http_serve, CmdExecutor};

#[derive(Debug, Parser)]
pub enum HttpSubcommand {
    /// Encode or decode base64 strings
    #[command(about = "")]
    Serve(HttpServeOpts),
}
impl CmdExecutor for HttpSubcommand {
    async fn exec(&self) -> anyhow::Result<()> {
        match self {
            Self::Serve(opts) => {
                process_http_serve(opts.dir.clone(), opts.port).await?;
            }
        }
        Ok(())
    }
}
#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
