mod error;

use std::{borrow::Cow, collections::HashMap, fs, io, mem, path::PathBuf};

use serde::Deserialize;

use crate::traits::append::Append;
pub use error::{NotFoundIn, TomlConfigError};

pub const CONFIG_FILENAME: &str = ".codecrc.toml";

#[derive(Deserialize, Debug, Default)]
pub struct PartialTomlConfig {
	#[serde(deserialize_with = "partial_deserialize_path")]
	pub templates_path: Option<PathBuf>,
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

		Self::try_build(base_config)
	}

	fn try_build(base: PartialTomlConfig) -> Result<Self, TomlConfigError> {
		let mut config = Self::try_from(base)?;
		let custom_variables = mem::replace(&mut config.variables, default_variables());
		config.variables.append(custom_variables);

		Ok(config)
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

fn default_variables() -> HashMap<String, String> {
	let mut map = HashMap::new();
	map.insert("namespace".into(), "app".into());

	map
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_have_default_variables() {
		let base = PartialTomlConfig {
			templates_path: Some(PathBuf::new()),
			..Default::default()
		};
		let config = TomlConfig::try_build(base).unwrap();

		assert_eq!(config.variables["namespace"], "app");
	}

	#[test]
	fn can_override_default_variables() {
		let variables = {
			let mut variables = HashMap::new();
			variables.insert("namespace".into(), "foo".into());
			variables
		};

		let base = PartialTomlConfig {
			variables: Some(variables),
			templates_path: Some(PathBuf::new()),
		};

		let config = TomlConfig::try_build(base).unwrap();

		assert_eq!(config.variables["namespace"], "foo");
	}
}
