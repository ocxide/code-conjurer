use std::path::PathBuf;

use miette::Diagnostic;

use crate::template::parse::error as parse_error;

#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum Error {
	#[error("Output name is invalid")]
	OutputNameInvalid,
	#[error("Template '{0}' in '{1}' was not accessible")]
	TemplateNotAccessible(String, PathBuf),

	#[error("Error while reading file '{0}'")]
	CouldNotRead(PathBuf),
	#[error("Error while writing file '{0}'")]
	CouldNotWrite(PathBuf),
	#[error("Could not open file '{0}'")]
	NotOpenable(PathBuf),

	#[error(transparent)]
	#[diagnostic(transparent)]
	Template(#[from] TemplateError),
}

#[derive(Debug, thiserror::Error, Diagnostic)]
enum TemplateError {
	#[diagnostic(
		code(ParamNotFound),
		help("Specify the param with the -p option. Eg. -p param=<value>")
	)]
	#[error("variable not found")]
	VariableNotFound {
		// For some reason, this attribute HAS to be named 'src', otherwise it wont compile
		// It think it is related to Diagnostic derive
		#[source_code]
		src: miette::NamedSource,
		#[label("here")]
		span: (usize, usize),
	},

	#[error("pipe '{pipe}' not found")]
	#[diagnostic(code(PipeNotFound), help("This pipe is not available."))]
	PipeNotFound {
		pipe: &'static str,
		#[source_code]
		src: miette::NamedSource,
		#[label("here")]
		span: (usize, usize),
	},
}

impl Error {
	pub fn from_parse_error(error: parse_error::Error, content: String, filename: String) -> Self {
		match error {
			parse_error::Error::External(_) => Self::CouldNotRead(PathBuf::from(filename)),
			parse_error::Error::Internal(parse_error::InternalError::PipeNotFound(pipe)) => {
				Self::Template(TemplateError::PipeNotFound {
					pipe,
					src: miette::NamedSource::new(filename, content),
					span: (1, 2),
				})
			}
			parse_error::Error::Internal(parse_error::InternalError::ParamNotFound(param)) => {
				Self::Template(TemplateError::VariableNotFound {
					src: miette::NamedSource::new(filename, content),
					span: (param.start, param.end - param.start),
				})
			}
		}
	}
}
