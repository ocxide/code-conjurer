use std::{collections::HashMap, path::PathBuf};

use crate::{config::Config, dir_browser::entry::Entry};

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Output name is invalid")]
	OutputNameInvalid,
	#[error("Template '{0}' in '{1}' was not accessible")]
	TemplateNotAccessible(String, PathBuf),
}

pub type FilesGenerated = Box<[PathBuf]>;

/// Arguments:
/// * `cli_variables`: arguments passed to cli like "namespace=foo, bar=baz",
/// * `template_name`: name of template stored in user files, like "ng-c", "ng-s", "rc-c", etc
/// * `output`: path where to generate the files,
pub fn generate(
	cli_variables: Vec<(String, String)>,
	template_name: String,
	output: PathBuf,
	mut config: Config,
) -> Result<FilesGenerated, Error> {
	let output_name = output
		.file_name()
		.and_then(|os_str| os_str.to_str())
		.ok_or(Error::OutputNameInvalid)?;

    attatch_variables(&mut config.toml_config.variables, cli_variables, output_name.to_owned());

	let template_path = config.toml_config.templates_path.join(&template_name);
	let template_entry = Entry::try_from(template_path)
		.map_err(|_| Error::TemplateNotAccessible(template_name, config.toml_config.templates_path))?;

	if let Entry::File(filename) = template_entry {
		return Ok([].into());
	}

	todo!()
}

fn attatch_variables(
	files_variables: &mut HashMap<String, String>,
	cli_variables: Vec<(String, String)>,
    output_name: String
) {
    if !files_variables.contains_key(&output_name) {
        files_variables.insert("name".into(), output_name);
    }

	for (key, value) in cli_variables {
		files_variables.insert(key, value);
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_attach_variables() {
       let mut files_variables = HashMap::new();
       files_variables.insert("namespace".into(), "app".into());
       let cli_variables = vec![
       ]; 
       let output_name = "foo".into();

       attatch_variables(&mut files_variables, cli_variables, output_name);
       assert_eq!(files_variables["namespace"], "app");
       assert_eq!(files_variables["name"], "foo");
    }
}

mod entry_reader {
	use std::{
		fs::{self, DirEntry, FileType, ReadDir},
		io,
		path::Path,
	};

	use crate::dir_browser::entry::Entry;

	pub trait EntryReader {
		type Entries: Iterator<Item = io::Result<Entry>>;

		fn read_entry(&self, path: impl AsRef<Path>) -> Result<Entry, std::io::Error>;

		fn read_entries(&self, path: impl AsRef<Path>) -> io::Result<Self::Entries>;
	}

	struct IoEntryReader;
	impl EntryReader for IoEntryReader {
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
}
