mod error;
mod toml_config;

use std::env::current_dir;
use std::env::current_exe;

use self::error::ConfigError;
use self::toml_config::TomlConfig;

#[derive(Debug)]
pub struct Config {
	pub toml_config: TomlConfig,
}

impl Config {
	pub fn try_new() -> Result<Self, ConfigError> {
		let toml_config = {
			match current_exe().ok() {
				Some(current_exe) => match current_dir().ok() {
					Some(current_dir) => TomlConfig::try_new(&[current_exe, current_dir]),
					None => TomlConfig::try_new(&[current_exe]),
				},
				None => return Err(ConfigError::CcoDirUnaccessable),
			}
		}?;

		Ok(Config { toml_config })
	}
}
