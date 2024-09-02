use clap::Parser;
use rust_cli::{process_csv, Cli, Commands};

fn main() -> anyhow::Result<()> {
    let opts = Cli::parse();
    match opts.cmd {
        Commands::Csv(csv_opts) => process_csv(&csv_opts.input, &csv_opts.output, &csv_opts.keys),
    }
}
