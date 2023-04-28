use std::{borrow::Cow, collections::HashMap, env::VarError, fs, io, path::PathBuf};

use serde::Deserialize;
use shellexpand::LookupError;

use crate::traits::append::Append;

pub const CONFIG_FILENAME: &str = ".coderc.toml";

#[derive(Debug, thiserror::Error)]
pub enum TomlConfigError {
	#[error("Toml config in {0:?} not found")]
	NotFound(PathBuf),
	#[error("Toml config in {0:?} unreadable")]
	Unreadable(PathBuf),
	#[error("Toml config unparseable: \n{0}")]
	Unparseable(#[from] toml::de::Error),

	#[error(transparent)]
	Lookup(#[from] LookupError<VarError>),
}

#[derive(Deserialize)]
struct RawTomlConfig {
	pub templates_path: String,
	pub variables: HashMap<String, String>,
}

pub struct TomlConfig {
	pub templates_path: PathBuf,
	pub variables: HashMap<String, String>,
}

impl TomlConfig {
	pub fn try_new(path: &PathBuf) -> Result<Self, TomlConfigError> {
		let path = path.join(CONFIG_FILENAME);
		let content = fs::read_to_string(path.clone()).map_err(|e| match e.kind() {
		    io::ErrorKind::NotFound => TomlConfigError::NotFound(path),
            _ => TomlConfigError::Unreadable(path)
		})?;

		let RawTomlConfig {
			templates_path,
			variables,
		} = toml::from_str::<RawTomlConfig>(&content)?;
		let expanded = shellexpand::full(&templates_path)?;
		let templates_path = match expanded {
			Cow::Owned(owned) => PathBuf::from(owned),
			Cow::Borrowed(borrowed) => PathBuf::from(borrowed),
		};

		let mut default_variables = default_variables();
		default_variables.append(variables);

		Ok(Self {
			templates_path,
			variables: default_variables,
		})
	}
}

fn default_variables() -> HashMap<String, String> {
	let mut map = HashMap::new();
	map.insert("namespace".into(), "app".into());

	map
}
