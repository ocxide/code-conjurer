mod error;

use std::{borrow::Cow, collections::HashMap, fs, path::PathBuf};

use serde::Deserialize;

pub use error::{NotFoundIn, TomlConfigError};

pub const CONFIG_FILENAME: &str = ".codecrc.toml";

#[derive(Deserialize, Debug, Default)]
pub struct PartialTomlConfig {
	#[serde(default)]
	#[serde(deserialize_with = "partial_deserialize_path")]
	pub templates_path: Option<PathBuf>,
	#[serde(default)]
	pub variables: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct TomlConfig {
	pub templates_path: PathBuf,
	pub variables: HashMap<String, String>,
}

fn partial_deserialize_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let may_path = Option::<String>::deserialize(deserializer)?;
	match may_path {
		None => Ok(None),
		Some(path) => {
			let expanded = match shellexpand::full(&path) {
				Ok(expanded) => expanded,
				Err(e) => return Err(serde::de::Error::custom(e)),
			};

			let templates_path = match expanded {
				Cow::Owned(owned) => PathBuf::from(owned),
				Cow::Borrowed(borrowed) => PathBuf::from(borrowed),
			};

			Ok(Some(templates_path))
		}
	}
}

impl TomlConfig {
	pub fn try_new(choices: &[PathBuf]) -> Result<Self, TomlConfigError> {
		let mut base_config = PartialTomlConfig::default();
		let mut found_any = false;

		let files = choices
			.iter()
			.map(|choice| choice.join(CONFIG_FILENAME))
			.collect::<Vec<_>>();

		for content in files.iter().flat_map(fs::read_to_string) {
			let added_config = toml::from_str::<PartialTomlConfig>(&content)?;
			if added_config.templates_path.is_some() {
				base_config.templates_path = added_config.templates_path;
			}

			if added_config.variables.is_some() {
				base_config.variables = added_config.variables;
			}

			found_any = true;
		}

		if !found_any {
			return Err(NotFoundIn(files.into()).into());
		}

		Self::try_from(base_config)
	}
}

impl TryFrom<PartialTomlConfig> for TomlConfig {
	type Error = TomlConfigError;

	fn try_from(value: PartialTomlConfig) -> Result<Self, Self::Error> {
		Ok(Self {
			templates_path: value
				.templates_path
				.ok_or_else(|| TomlConfigError::MissingField("templates_path"))?,
			variables: value.variables.unwrap_or_default(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reads_correctly() {
		let paths = [
			PathBuf::from("./tests/mock-config/foo/"),
			PathBuf::from("./tests/mock-config/bar/"),
		];

		let config = TomlConfig::try_new(&paths).unwrap();
		assert_eq!(config.variables["namespace"], "foo");
	}
}
