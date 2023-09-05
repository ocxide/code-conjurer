use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::template::parse::error::ParamNotFound;

#[derive(Error, Debug, Diagnostic)]
#[error("Param Not found in template")]
#[diagnostic(
	code(ParamNotFound),
	help("Specify the param with the -p option. Eg. -p param=<value>")
)]
pub struct ParamNotFoundDiagnostic {
	#[source_code]
	src: NamedSource,
	#[label("This param here")]
	param: SourceSpan,
}

impl ParamNotFoundDiagnostic {
	pub fn new(filename: impl AsRef<str>, content: impl Into<String>, range: (usize, usize)) -> Self {
		let source = NamedSource::new(filename, content.into());
		let param = range.into();

		Self { src: source, param }
	}

	pub fn from_error(error: ParamNotFound, filename: impl AsRef<str>) -> Self {
		let range = (error.start, error.end - error.start);
		let content = "";

		Self::new(filename, content, range)
	}
}
