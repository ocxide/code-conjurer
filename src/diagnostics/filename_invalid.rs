use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Filename Invalid")]
#[diagnostic(code(io::FilenameInvalid))]
pub struct FilenameInvalid {
	#[source_code]
	src: String,
	#[label("Invalid")]
	invalid: SourceSpan,
}

impl FilenameInvalid {
	pub fn new(filename: impl Into<String>) -> Self {
		let filename = filename.into();
		let len = filename.len();

		Self {
			src: filename,
			invalid: (0, len).into(),
		}
	}
}
