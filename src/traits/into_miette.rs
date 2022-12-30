use std::{ffi::OsStr, io, path::PathBuf};

use crate::diagnostics::{
	file_not_found::FileNotFoundDiagnostic, filename_invalid::FilenameInvalidDiagnostic,
};

pub trait IntoMiette<'a> {
	type T;
	type DiagnosticMetadata;

	fn into_miette(self, metadata: Self::DiagnosticMetadata) -> miette::Result<Self::T>;
}

impl<'a, T> IntoMiette<'a> for io::Result<T> {
	type T = T;
	type DiagnosticMetadata = (&'a PathBuf, &'static str);

	fn into_miette(self, (path, alias): Self::DiagnosticMetadata) -> miette::Result<Self::T> {
		match self {
			Ok(value) => Ok(value),
			Err(_) => {
				let p = path
					.clone()
					.into_os_string()
					.into_string()
					.map_err(|_| FilenameInvalidDiagnostic::new(alias))?;
				Err(FileNotFoundDiagnostic::new(p))?
			}
		}
	}
}

impl<'a> IntoMiette<'a> for Option<&OsStr> {
	type T = String;
	type DiagnosticMetadata = &'static str;

	fn into_miette(self, metadata: Self::DiagnosticMetadata) -> miette::Result<String> {
		let os_name = self.ok_or_else(|| FilenameInvalidDiagnostic::new(metadata))?;
		let name = os_name
			.to_str()
			.ok_or_else(|| FilenameInvalidDiagnostic::new(metadata))?
			.to_owned();

		Ok(name)
	}
}
