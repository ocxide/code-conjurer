mod error;
pub mod toml_config;

use std::env::current_dir;
use std::env::current_exe;
use std::path::PathBuf;

use self::error::ConfigError;
use self::toml_config::TomlConfig;

#[derive(Debug)]
pub struct Config {
	pub toml_config: TomlConfig,
}

impl Config {
	pub fn try_new() -> Result<Self, ConfigError> {
		let routes = [current_exe_dir(), current_dir()]
			.into_iter()
			.flatten()
			.collect::<Vec<_>>();

		if routes.len() == 0 {
			return Err(ConfigError::DirectoriesUnaccessable);
		}

		let toml_config = TomlConfig::try_new(&routes)?;

		Ok(Config { toml_config })
	}
}

fn current_exe_dir() -> std::io::Result<PathBuf> {
	let exe_path = current_exe()?;
	let exe_dir = exe_path
		.parent()
		.ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;

	Ok(exe_dir.to_owned())
}
