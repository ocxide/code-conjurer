use miette::{Diagnostic, NamedSource};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Filename Invalid")]
#[diagnostic(code(io::FilenameInvalid))]
pub struct FilenameInvalid {
	#[source_code]
	filename: NamedSource,
}

impl FilenameInvalid {
	pub fn new(filename: NamedSource) -> Self {
		Self { filename }
	}
}
