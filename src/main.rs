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
use cli::GenerateCommand;
use commands::generate::recursive_generate;
use miette::IntoDiagnostic;
use terminal::create_file::create_file;

fn main() -> miette::Result<()> {
	let cli = Cli::parse();

	match cli.commands {
		Commands::Generate(GenerateCommand {
			template,
			path,
			params,
		}) => {
			let output = match path {
				None => create_file().into_diagnostic()?,
				Some(path) => path.into(),
			};
			dbg!(&output);

			recursive_generate(params, template, output)?;
			Ok(()) as miette::Result<()>
		}
	}?;

	Ok(())
}
