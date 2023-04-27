use std::{
	fs::{self, File},
	io,
	path::Path,
};

pub fn create_file(path: impl AsRef<Path>) -> io::Result<File> {
	let path = path.as_ref();
	let parent = path.parent();

	if let Some(parent) = parent {
		fs::create_dir_all(parent)?;
	}

	let file = File::create(path)?;
	Ok(file)
}
