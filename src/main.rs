mod cli;
mod template;
mod commands;
mod diagnostics;

use clap::Parser;
use crate::cli::{ Cli, Commands };
use crate::commands::generate::generate;

fn main() -> miette::Result<()> {
	let cli = Cli::parse();

	match cli.commands {
		Commands::Generate(c) => generate(c)
	}?;

	Ok(())
}
