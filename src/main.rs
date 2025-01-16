use clap::Parser;
use rcli::{
    process_base64_decode, process_base64_encode, process_csv, process_genpass, process_http_serve,
    process_sign, process_verify, Base64Subcommand, HttpSubcommand, Opts, SubCommand,
    TextSignFormat, TextSubcommand,
};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
        SubCommand::Text(subcmd) => match subcmd {
            TextSubcommand::Sign(opts) => {
                match opts.format {
                    TextSignFormat::Blake3 => {
                        process_sign(&opts.input, &opts.key, opts.format)?;
                        println!("{:?}", opts)
                    }
                    TextSignFormat::Ed25519 => todo!(),
                };
            }
            TextSubcommand::Verify(opts) => {
                // let output = process_base64_decode(&input)?;
                println!("{:?}", opts);
                match opts.format {
                    TextSignFormat::Blake3 => {
                        process_verify(&opts.input, &opts.key, "", opts.format)?;
                    }
                    TextSignFormat::Ed25519 => todo!(),
                };
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubcommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}

#[test]
fn test() {
    // assert_eq!(2 + 2, 4);
}
