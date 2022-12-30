use std::{io, path::PathBuf};

use crate::traits::{try_default::TryDefault, try_from::MyTryInto};

use super::entry::Entry;

pub struct DirBrowser {
	entries: Vec<Entry>,
	path: PathBuf,
}

impl DirBrowser {
	pub fn new(path: PathBuf) -> io::Result<Self> {
		let entries: Vec<Entry> = path
			.read_dir()?
			.flatten()
			.map(|entry| entry.path().my_try_into())
			.collect::<Result<_, _>>()?;
		Ok(DirBrowser { entries, path })
	}

	pub fn enter(&mut self, i: usize) -> io::Result<()> {
		let entry = self.entries.get(i).ok_or(io::ErrorKind::NotFound)?;
		let dirname = match entry {
			Entry::Directory(name) => name,
			_ => return Err(io::Error::new(io::ErrorKind::NotFound, "Not a director")),
		};

		let path = self.path.join(dirname);
		*self = Self::new(path)?;

		Ok(())
	}

	pub fn back(&mut self) -> io::Result<()> {
		let path = self.path.join("..");
		*self = Self::new(path)?;

		Ok(())
	}

	pub fn read_dir(&self) -> impl Iterator<Item = &Entry> {
		self.entries.iter()
	}

	pub fn get_path(&self) -> &PathBuf {
		&self.path
	}
}

impl TryDefault for DirBrowser {
	type Error = io::Error;

	fn try_default() -> Result<Self, Self::Error> {
		let path = PathBuf::from(".");
		Self::new(path)
	}
}
