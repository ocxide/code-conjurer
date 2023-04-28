use clap::{Args, Parser, Subcommand};
use params_parser::parse_key_val;

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	/// Prints path of the exe
	#[clap(alias = "p")]
	Path,
	/// Generates a file code from a given template
	#[clap(alias = "g")]
	Generate(GenerateCommand),
    /// List templates of the current config
    List,
}

#[derive(Debug, Args)]
pub struct GenerateCommand {
	/// Template name or path to generate
	pub template: String,

	/// Path for generating
	pub path: Option<String>,
	// / Output file name
	// pub output: String,
	/// Aditional parameters for the template output
	#[arg(short = 'p', value_parser = parse_key_val::<String, String>)]
	pub params: Vec<(String, String)>,
}

mod params_parser {
	use std::error::Error;

	pub fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
	where
		T: std::str::FromStr,
		T::Err: Error + Send + Sync + 'static,
		U: std::str::FromStr,
		U::Err: Error + Send + Sync + 'static,
	{
		let pos = s
			.find('=')
			.ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
		Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
	}
}
