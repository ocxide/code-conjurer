use std::{
	fs::{self, DirEntry, FileType, ReadDir},
	io,
	path::Path,
};

use crate::dir_browser::entry::Entry;

pub trait FileSystemReader {
	type Entries: Iterator<Item = io::Result<Entry>>;

	fn read_entry(&self, path: impl AsRef<Path>) -> Result<Entry, std::io::Error>;

	fn read_entries(&self, path: impl AsRef<Path>) -> io::Result<Self::Entries>;
}

pub struct IoEntryReader;
impl FileSystemReader for IoEntryReader {
	type Entries = IoEntryDirectoryReader;

	fn read_entry(&self, path: impl AsRef<Path>) -> Result<Entry, std::io::Error> {
		read_entry(path)
	}

	fn read_entries(&self, path: impl AsRef<Path>) -> io::Result<Self::Entries> {
		fs::read_dir(path).map(|dir_iter| IoEntryDirectoryReader(dir_iter))
	}
}

pub struct IoEntryDirectoryReader(ReadDir);
impl Iterator for IoEntryDirectoryReader {
	type Item = io::Result<Entry>;

	fn next(&mut self) -> Option<Self::Item> {
		let res = self.0.next()?;
		Some(entry_from_dir(res))
	}
}

fn read_entry(path: impl AsRef<Path>) -> Result<Entry, std::io::Error> {
	let path = path.as_ref();
	let file_type = path.metadata()?.file_type();
	let filename = path
		.file_name()
		.ok_or(std::io::ErrorKind::Other)?
		.to_str()
		.ok_or(std::io::ErrorKind::Other)?
		.to_owned();

	let entry = if file_type.is_dir() {
		Entry::Directory(filename)
	} else if file_type.is_file() {
		Entry::File(filename)
	} else {
		Entry::symlink(filename, fs::read_link(path)?)
	};

	Ok(entry)
}

fn create_entry(path: &Path, file_name: Option<&str>, file_type: FileType) -> io::Result<Entry> {
	let filename = file_name.ok_or(std::io::ErrorKind::Other)?.to_owned();

	let entry = if file_type.is_dir() {
		Entry::Directory(filename)
	} else if file_type.is_file() {
		Entry::File(filename)
	} else {
		Entry::symlink(filename, fs::read_link(path)?)
	};

	Ok(entry)
}

fn entry_from_dir(dir_entry: io::Result<DirEntry>) -> io::Result<Entry> {
	let dir_entry = dir_entry?;
	create_entry(
		&dir_entry.path(),
		dir_entry.file_name().to_str(),
		dir_entry.file_type()?,
	)
}
