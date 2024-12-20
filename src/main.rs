mod cli;
mod ok;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    let res = match args.cmd {
        cli::TdxCommand::Ok => ok::check_for_guest_device(),
    };

    res
}
