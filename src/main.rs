// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.output_format)
            };
            process_csv(&opts.input, output, opts.output_format)?;
        }
        SubCommand::Genpass(opts) => {
            println!(
                "{}",
                process_genpass(
                    opts.length,
                    opts.uppercase,
                    opts.lowercase,
                    opts.numbers,
                    opts.symbols
                )
            );
        }
    }

    Ok(())
}
