mod cli;
mod commands;
mod diagnostics;
mod dir_browser;
mod path;
mod template;
mod terminal;
mod traits;

use crate::cli::{Cli, Commands};
use clap::Parser;
use commands::generate::recursive_generate;
use miette::IntoDiagnostic;
use terminal::create_file::create_file;

fn main() -> miette::Result<()> {
	let cli = Cli::parse();

	match cli.commands {
		Commands::Generate(c) => {
			let output = create_file().into_diagnostic()?;
			recursive_generate(c, output)?;
			Ok(()) as miette::Result<()>
		}
	}?;

	Ok(())
}
