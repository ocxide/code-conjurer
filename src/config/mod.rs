mod error;

use std::fs::read_to_string;
use std::path::PathBuf;
use std::{borrow::Cow, env::current_exe};

use serde::Deserialize;

use self::error::ConfigError;

const CONFIG_PATH: &str = "config.toml";

#[derive(Deserialize)]
struct TomlConfig {
	pub templates_path: String,
}

pub struct Config {
	pub templates_path: PathBuf,
}

impl Config {
	pub fn try_new() -> Result<Self, ConfigError> {
		let config_toml_path = current_exe()
			.map_err(|_| ConfigError::CcoDirUnaccessable)?
			.parent()
			.ok_or_else(|| ConfigError::CcoDirUnaccessable)?
			.join(CONFIG_PATH);

		let toml_content =
			read_to_string(config_toml_path).map_err(|_| ConfigError::ConfigTomlNotFound)?;

		let TomlConfig { templates_path } =
			toml::from_str::<TomlConfig>(&toml_content).map_err(|_| ConfigError::TomlUnparseable)?;

		let expanded = shellexpand::full(&templates_path)?;
		let templates_path = match expanded {
			Cow::Owned(owned) => PathBuf::from(owned),
			Cow::Borrowed(borrowed) => PathBuf::from(borrowed),
		};

		Ok(Config { templates_path })
	}
}
