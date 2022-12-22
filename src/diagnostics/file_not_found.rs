use miette::{Diagnostic};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("File not found")]
#[diagnostic(
	code(io::FileNotFound),
)]
pub struct FileNotFoundDiagnostic {
	#[source_code]
  path: String,
}

impl FileNotFoundDiagnostic {
	pub fn from_path(path: &str) -> Self {
		Self { path: format!("Path: {}", path) }
	}
}