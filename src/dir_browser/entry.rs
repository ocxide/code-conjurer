use std::{
	fs::{self, DirEntry},
	io,
	path::PathBuf,
};

#[derive(Debug)]
pub enum Entry {
	File(String),
	Directory(String),
	Symlink(Symlink),
}

#[derive(Debug)]
pub struct Symlink {
	pub name: String,
	pub link: PathBuf,
}

impl TryFrom<DirEntry> for Entry {
	type Error = io::Error;

	fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
		let metadata = value.metadata()?;
		let name = value
			.file_name()
			.to_str()
			.ok_or(io::ErrorKind::InvalidData)?
			.to_owned();

		if metadata.is_file() {
			Ok(Entry::File(name))
		} else if metadata.is_dir() {
			Ok(Entry::Directory(name))
		} else {
			let path = value.path();
			let link = fs::read_link(path)?;
			Ok(Entry::Symlink(Symlink { name, link }))
		}
	}
}
