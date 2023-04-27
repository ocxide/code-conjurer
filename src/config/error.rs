use std::error::Error;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ConfigError {
	#[error("Config file 'config.toml' was not found")]
	ConfigTomlNotFound,

	#[error("Config file 'config.toml' unparseable")]
	TomlUnparseable,

	#[error(transparent)]
	TemplatePathError(#[from] TemplatePathError),
}

#[derive(Debug)]
pub struct TemplatePathError;
impl std::fmt::Display for TemplatePathError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match std::env::current_dir()
			.ok()
			.map(|dir| dir.into_os_string().into_string().ok())
			.flatten()
		{
			Some(dir) => {
				write!(
					f,
					"Invalid templates_path in config file '{dir}/config.toml'"
				)
			}
			_ => write!(f, "Invalid templates_path in config file 'config.toml'"),
		}
	}
}

impl Error for TemplatePathError {}
