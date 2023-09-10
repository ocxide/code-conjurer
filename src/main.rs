mod cli;
mod commands;
mod config;
mod diagnostics;
mod dir_browser;
mod template;
mod terminal;
mod traits;

mod io;

use crate::cli::{Cli, Commands};
use clap::Parser;
use cli::GenerateCommand;
use commands::{generate, list::list, path::print_path};
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
		}
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

			match generate::generate(params, template, output, config) {
				Ok(files) => {
                    println!("Generated {} files:", files.len());
					files.into_iter().for_each(|path| {
						println!("{} ✓", path.to_string_lossy());
					});
					Ok(())
				}
				Err(e) => Err(e.into()),
			}
		}
	}
}
