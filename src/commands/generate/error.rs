use std::{fmt::Display, path::PathBuf};

use miette::Diagnostic;

use crate::template::parse::error as parse_error;

#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum Error {
	#[error("Output name is invalid")]
	OutputNameInvalid,

	#[error("Error while reading file '{0}'")]
	CouldNotRead(PathBuf),
	#[error("Error while writing file '{0}'")]
	CouldNotWrite(PathBuf),
	#[error("Could not open file '{0}'")]
	NotOpenable(PathBuf),

	#[error("Template '{template_name}' in {} is invalid", templates_dir.to_string_lossy())]
	#[diagnostic(
        code(template_error::TemplateNotValid), 
        help("Templates must be directories.")
    )]
	TemplateNotValid {
		template_name: String,
		templates_dir: PathBuf,
	},

	#[error(transparent)]
	#[diagnostic(transparent)]
	Template(#[from] TemplateError),

	#[error(transparent)]
	#[diagnostic(transparent)]
	TemplateNotFound(TemplateNotFoundError),
}

impl Error {
	pub fn template_not_found(name: String, templates_path: PathBuf) -> Self {
		Self::TemplateNotFound(TemplateNotFoundError {
			templates_dir: templates_path,
			template: Box::from(name),
		})
	}

	pub fn template_invalid(template_name: String, templates_dir: PathBuf) -> Self {
		Self::TemplateNotValid {
			template_name,
			templates_dir,
		}
	}
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Template '{template}' was not found in {}.", templates_dir.to_string_lossy())]
#[diagnostic(
    code(TemplateNotFoundError),
    help("You might refer to one of theese templates.\nAvailable templates: {}", self.templates())
)]
pub struct TemplateNotFoundError {
	template: Box<str>,
	templates_dir: PathBuf,
}

impl TemplateNotFoundError {
	pub fn templates<'a>(&'a self) -> TemplateDisplay<'a> {
		TemplateDisplay(&self.templates_dir)
	}
}

pub struct TemplateDisplay<'a>(&'a std::path::Path);

impl<'a> Display for TemplateDisplay<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let read_dir = match std::fs::read_dir(self.0) {
			Ok(read_dir) => read_dir,
			Err(e) => return write!(f, "Could not read directory, {}", e),
		};

		writeln!(f)?;

		for entry in read_dir {
			let entry = match entry {
				Ok(entry) => entry,
				Err(_) => {
					let _ = f.write_str("<Could not read entry>\n");
					continue;
				}
			};

			let filename = entry.file_name();
			writeln!(f, "- {}", filename.to_string_lossy())?;
		}

		Ok(())
	}
}

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum TemplateError {
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
