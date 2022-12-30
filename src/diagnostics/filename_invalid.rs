use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("{file_alias}: Filename Invalid")]
#[diagnostic(code(io::FilenameInvalid))]
pub struct FilenameInvalidDiagnostic {
	file_alias: String,
}

impl FilenameInvalidDiagnostic {
	pub fn new(file_alias: impl Into<String>) -> Self {
		Self {
			file_alias: file_alias.into(),
		}
	}
}
