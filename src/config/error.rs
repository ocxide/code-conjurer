use std::env::VarError;

use shellexpand::LookupError;

fn config_path() -> String {
	let dir = std::env::current_exe();
	let dir = match &dir {
		Ok(dir) => dir
			.parent()
			.map(|path| path.to_str())
			.flatten()
			.unwrap_or_else(|| "{unknown}"),
		_ => "{unknown}",
	};

	format!("{dir}/config.toml")
}

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ConfigError {
	#[error("Dir of cco executable is unaccessable")]
	CcoDirUnaccessable,
	#[error("Config file '{}' was not found", config_path())]
	ConfigTomlNotFound,

	#[error("Config file 'config.toml' unparseable")]
	TomlUnparseable,

	#[error(transparent)]
	TemplatePathError(#[from] LookupError<VarError>),
}
