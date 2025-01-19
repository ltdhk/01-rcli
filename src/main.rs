use clap::Parser;
use rcli::{CmdExecutor, Opts};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    opts.cmd.exec().await?;
    Ok(())
}

#[test]
fn test() {
    // assert_eq!(2 + 2, 4);
}
