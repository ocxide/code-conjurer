use super::toml_config::TomlConfigError;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ConfigError {
	#[error("None of cco executable or dir was accessable")]
	DirectoriesUnaccessable,

	#[error(transparent)]
	TomlConfigError(#[from] TomlConfigError),
}
