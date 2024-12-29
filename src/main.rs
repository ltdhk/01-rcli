use clap::Parser;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_genpass, Base64Subcommand,
    Opts, SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output: String = if let Some(output) = csv_opts.output {
                output.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
        SubCommand::GenPass(genpass_opts) => {
            process_genpass(&genpass_opts)?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                process_base64_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                // let output = process_base64_decode(&input)?;
                process_base64_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}

#[test]
fn test() {
    // assert_eq!(2 + 2, 4);
}
