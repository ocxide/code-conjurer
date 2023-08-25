use std::path::PathBuf;

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
///
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

	if !config.toml_config.variables.contains_key("name") {
		config
			.toml_config
			.variables
			.insert("name".into(), output_name.to_owned());
	}

	for (key, value) in cli_variables {
		config.toml_config.variables.insert(key, value);
	}

	let template_path = config.toml_config.templates_path.join(&template_name);
	let template_entry = Entry::try_from(template_path)
		.map_err(|_| Error::TemplateNotAccessible(template_name, config.toml_config.templates_path))?;

	if let Entry::File(filename) = template_entry {
		return Ok([].into());
	}

	todo!()
}

mod entry_reader {
	use std::path::{Path, PathBuf};

	use crate::dir_browser::entry::Entry;

	pub trait EntryReader {
		fn read_entry(&self, path: impl AsRef<Path>) -> Result<Entry, std::io::Error>;
	}
}
