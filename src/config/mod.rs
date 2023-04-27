mod error;

use std::borrow::Cow;
use std::fs::read_to_string;
use std::path::PathBuf;

use serde::Deserialize;

use self::error::{ConfigError, TemplatePathError};

const CONFIG_PATH: &str = "./config.toml";

#[derive(Deserialize)]
struct TomlConfig {
	pub templates_path: String,
}

pub struct Config {
	pub templates_path: PathBuf,
}

impl Config {
	pub fn try_new() -> Result<Self, ConfigError> {
		let toml_content = read_to_string(CONFIG_PATH).map_err(|_| ConfigError::ConfigTomlNotFound)?;
		let TomlConfig { templates_path } =
			toml::from_str::<TomlConfig>(&toml_content).map_err(|_| ConfigError::TomlUnparseable)?;

		let expanded =
			shellexpand::full(&templates_path).map_err(|_| TemplatePathError)?;
		let templates_path = match expanded {
			Cow::Owned(owned) => PathBuf::from(owned),
			Cow::Borrowed(borrowed) => PathBuf::from(borrowed),
		};

		Ok(Config { templates_path })
	}
}
