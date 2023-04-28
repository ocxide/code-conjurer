mod error;
mod toml_config;

use std::env::current_dir;
use std::env::current_exe;

use self::error::ConfigError;
use self::toml_config::TomlConfig;

pub struct Config {
	pub toml_config: TomlConfig,
}

impl Config {
	pub fn try_new() -> Result<Self, ConfigError> {
		let config_toml_path = current_exe()
			.ok()
			.and_then(|path| path.parent().map(|path| path.to_owned()))
			.or_else(|| current_dir().ok())
			.ok_or_else(|| ConfigError::CcoDirUnaccessable)?;

		let toml_config = TomlConfig::try_new(&config_toml_path)?;
		Ok(Config { toml_config })
	}
}
