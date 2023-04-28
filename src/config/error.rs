use super::toml_config::TomlConfigError;

#[derive(thiserror::Error, Debug, miette::Diagnostic)]
pub enum ConfigError {
	#[error("Dir of cco executable is unaccessable")]
	CcoDirUnaccessable,

	#[error(transparent)]
    TomlConfigError(#[from] TomlConfigError),
}
