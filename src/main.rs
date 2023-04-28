mod cli;
mod commands;
mod config;
mod diagnostics;
mod dir_browser;
mod path;
mod template;
mod terminal;
mod traits;

use crate::cli::{Cli, Commands};
use clap::Parser;
use cli::GenerateCommand;
use commands::{generate::recursive_generate, path::print_path, list::list};
use config::Config;
use miette::IntoDiagnostic;
use terminal::create_file::create_file;

fn main() -> miette::Result<()> {
	let config = Config::try_new();
	let cli = Cli::parse();

	match cli.commands {
        Commands::List => {
            let config = config?;
            list(&config).into_diagnostic()
        },
		Commands::Path => print_path(),
		Commands::Generate(GenerateCommand {
			template,
			path,
			params,
		}) => {
			let config = config?;
			let output = match path {
				None => create_file().into_diagnostic()?,
				Some(path) => path.into(),
			};

			recursive_generate(params, template, output, &config)
		}
	}
}
