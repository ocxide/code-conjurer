mod cli;
mod commands;
mod diagnostics;
mod template;

use crate::cli::{Cli, Commands};
use crate::commands::generate::generate;
use clap::Parser;

fn main() -> miette::Result<()> {
	let cli = Cli::parse();

	match cli.commands {
		Commands::Generate(c) => generate(c),
	}?;

	Ok(())
}
