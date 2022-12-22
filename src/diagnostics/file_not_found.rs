use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("File not found")]
#[diagnostic(code(io::FileNotFound))]
pub struct FileNotFoundDiagnostic {
	#[source_code]
	path: String,
	#[label = "File not found"]
	not_found: SourceSpan,
}

impl FileNotFoundDiagnostic {
	pub fn from_path(path: impl Into<String>) -> Self {
		let path = path.into();
		let len = path.len();

		Self {
			path,
			not_found: (0, len).into()
		}
	}
}
