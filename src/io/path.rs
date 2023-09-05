use std::{ffi::OsString, path::PathBuf};

/// A path to a file with a known name
/// Atributes
/// * `name`: The name of the file
/// * `dir_path`: The path to the file, containing the name
#[derive(Debug, Clone)]
pub struct NamedPathBuf {
	pub filename: OsString,
	pub pathbuf: PathBuf,
}

impl TryFrom<PathBuf> for NamedPathBuf {
	type Error = std::io::Error;

	fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
		let filename = value.file_name().ok_or(std::io::ErrorKind::Other)?;

		let named_path = NamedPathBuf {
			filename: filename.to_os_string(),
			pathbuf: value,
		};

		Ok(named_path)
	}
}

impl NamedPathBuf {
	pub fn new(pathbuf: PathBuf, filename: OsString) -> Self {
		Self { filename, pathbuf }
	}
}
