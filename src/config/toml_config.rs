use std::{borrow::Cow, collections::HashMap, env::VarError, fs, io, path::PathBuf};

use serde::Deserialize;
use shellexpand::LookupError;

use crate::traits::append::Append;

pub const CONFIG_FILENAME: &str = ".coderc.toml";

#[derive(Debug, thiserror::Error)]
pub enum TomlConfigError {
	#[error("Toml config in {0:?} not found")]
	NotFound(PathBuf),

	#[error(transparent)]
	NotFoundIn(#[from] NotFoundIn),

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

#[derive(Debug)]
pub struct TomlConfig {
	pub templates_path: PathBuf,
	pub variables: HashMap<String, String>,
}

impl TomlConfig {
	pub fn try_new(choices: &[PathBuf]) -> Result<Self, TomlConfigError> {
		let path = get_path(choices, CONFIG_FILENAME)?;
		let content = fs::read_to_string(path.clone()).map_err(|e| match e.kind() {
			io::ErrorKind::NotFound => TomlConfigError::NotFound(path.clone()),
			_ => TomlConfigError::Unreadable(path.clone()),
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

		if !templates_path.exists() {
			return Err(TomlConfigError::NotFound(templates_path));
		}

		let mut default_variables = default_variables();
		default_variables.append(variables);

		Ok(Self {
			templates_path,
			variables: default_variables,
		})
	}
}

fn get_path(choices: &[PathBuf], path: &str) -> Result<PathBuf, NotFoundIn> {
	let mut not_found = vec![];
	for path in choices.into_iter().map(|choice| choice.join(path)) {
		if path.exists() {
			return Ok(path);
		}
		not_found.push(path)
	}

	Err(NotFoundIn(not_found))
}

#[derive(Debug)]
pub struct NotFoundIn(Vec<PathBuf>);
impl std::error::Error for NotFoundIn {}
impl std::fmt::Display for NotFoundIn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut message = "Toml config not found in:\n".to_string();
		self
			.0
			.iter()
			.map(|path| {
				let mut displayed = path.display().to_string();
				displayed.push_str(" or\n");
				displayed
			})
			.for_each(|line| message.push_str(&line));

		message.pop(); // \n
		message.pop(); // r
		message.pop(); // o
		message.pop(); // %20

		f.write_str(&message)
	}
}

fn default_variables() -> HashMap<String, String> {
	let mut map = HashMap::new();
	map.insert("namespace".into(), "app".into());

	map
}
