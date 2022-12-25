mod cli;
mod commands;
mod diagnostics;
mod template;
mod terminal;

use crate::cli::{Cli, Commands};
use crate::commands::generate::generate;
use clap::Parser;
use miette::IntoDiagnostic;
use terminal::create_file::create_file;

fn main() -> miette::Result<()> {
	let cli = Cli::parse();

	match cli.commands {
		Commands::Generate(c) => {
			let output = create_file().into_diagnostic()?;
			generate(c, output)?;
			Ok(()) as miette::Result<()>
		}
	}?;

	Ok(())
}
