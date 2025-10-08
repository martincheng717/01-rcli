use clap::Parser;
use rcli::{ process_csv, Opts, SubCommand, process_genpass };

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) =>{
            process_genpass(opts)?;
            println!("Generate password:{:?}", opts)
        }
    }
    Ok(())
}
