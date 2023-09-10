mod error;

use std::{
	collections::HashMap,
	ffi::OsString,
	fs::{self, FileType},
	io::{BufRead, BufReader, BufWriter, Write},
	os::unix::prelude::OsStringExt,
	path::PathBuf,
};

use crate::{
	config::Config,
	io::path::NamedPathBuf,
	template::parse::{DefaultTemplateParse, TemplateParse},
};

use error::Error;

pub type FilesGenerated = Box<[PathBuf]>;

/// Arguments:
/// * `cli_variables`: arguments passed to cli like "namespace=foo, bar=baz",
/// * `template_name`: name of template stored in user files, like "ng-c", "ng-s", "rc-c", etc
/// * `output`: path where to generate the files,
pub fn generate(
	cli_variables: Vec<(String, String)>,
	template_name: String,
	mut output: PathBuf,
	mut config: Config,
) -> Result<FilesGenerated, Error> {
	let output_name = output
		.file_name()
		.and_then(|os_str| os_str.to_str())
		.ok_or(Error::OutputNameInvalid)?;

	attatch_variables(
		&mut config.toml_config.variables,
		cli_variables,
		output_name.to_owned(),
	);

	let template_path = config.toml_config.templates_path.join(&template_name);
	// Use a match to avoid borrow checker issues
	let template_file_metadata = match template_path.metadata() {
		Ok(metadata) => metadata,
		Err(_) => {
			let error = Error::template_not_found(template_name, config.toml_config.templates_path);
			return Err(error);
		}
	};

	let mut files_generated = vec![];
	let parser = DefaultTemplateParse::with_vars(config.toml_config.variables);

	if !template_file_metadata.file_type().is_dir() {
		return Err(Error::template_invalid(template_name, template_path));
	}

	// Create the files in the parent output directory
	if !output.pop() {
		return Err(Error::OutputNameInvalid); // TODO: Better error
	}

	if fs::create_dir_all(&output).is_err() {
		return Err(Error::CouldNotWrite(output)); // Find better error
	}

	generate_dir(template_path, output, &mut files_generated, &parser)?;

	Ok(files_generated.into_boxed_slice())
}

fn recursive_generate<T: TemplateParse>(
	template_path: NamedPathBuf,
	template_filetype: FileType,
	output: NamedPathBuf,
	files_generated: &mut Vec<PathBuf>,

	template_parser: &T,
) -> Result<(), Error> {
	if template_filetype.is_file() {
		generate_file(
			template_path.pathbuf,
			output.pathbuf,
			files_generated,
			template_parser,
		)
	} else if template_filetype.is_dir() {
		if fs::create_dir(&output.pathbuf).is_err() {
			return Err(Error::CouldNotWrite(output.pathbuf)); // Find better error
		}

		generate_dir(
			template_path.pathbuf,
			output.pathbuf,
			files_generated,
			template_parser,
		)
	} else {
		unimplemented!("Symlinks are not supported yet! :(");
	}
}

fn generate_dir<T: TemplateParse>(
	template_dir: PathBuf,
	output: PathBuf,
	files_generated: &mut Vec<PathBuf>,
	template_parser: &T,
) -> Result<(), Error> {
	let read_dir = match template_dir.read_dir() {
		Ok(read_dir) => read_dir,
		Err(_) => {
			let error = Error::CouldNotRead(template_dir);
			return Err(error);
		}
	};

	for entry in read_dir {
		let entry = match entry {
			Ok(entry) => entry,
			Err(_) => {
				let error = Error::CouldNotRead(template_dir);
				return Err(error);
			}
		};

		let os_filename = entry.file_name();
		let filename = match os_filename.to_str() {
			Some(filename) => filename,
			None => {
				let error = Error::CouldNotRead(entry.path()); // TODO: Better error
				return Err(error);
			}
		};

		let mut parsed_filename = vec![];
		if let Err(e) = template_parser.parse(filename, &mut parsed_filename) {
			let error = Error::from_parse_error(e, filename.to_string(), filename.to_string()); // TODO
			return Err(error);
		};

		let parsed_filename = OsString::from_vec(parsed_filename);

		// Do the filetype call in this scope because it is almost free in most platforms
		let filetype = match entry.file_type() {
			Ok(filetype) => filetype,
			Err(_) => {
				let error = Error::CouldNotRead(template_dir);
				return Err(error);
			}
		};

		let output_named_path =
			NamedPathBuf::new(output.join(&parsed_filename), parsed_filename.clone());

		recursive_generate(
			NamedPathBuf::new(entry.path(), os_filename),
			filetype,
			output_named_path,
			files_generated,
			template_parser,
		)?;
	}

	Ok(())
}

fn generate_file<T: TemplateParse>(
	template_filename: PathBuf,
	output_filename: PathBuf,
	generated_files: &mut Vec<PathBuf>,
	template_parser: &T,
) -> Result<(), Error> {
	let template_file = match std::fs::File::open(&template_filename) {
		Ok(file) => file,
		Err(_) => {
			let error = Error::NotOpenable(template_filename);
			return Err(error);
		}
	};
	let template_file = BufReader::new(template_file);

	let output_file = match std::fs::File::create(&output_filename) {
		Ok(file) => file,
		Err(_) => {
			let error = Error::CouldNotWrite(output_filename);
			return Err(error);
		}
	};
	let mut output_file = BufWriter::new(output_file);

	for result in template_file.lines() {
		let line = match result {
			Ok(line) => line,
			Err(_) => {
				let error = Error::CouldNotRead(template_filename);
				return Err(error);
			}
		};

		if let Err(e) = template_parser.parse(&line, &mut output_file) {
			let error = Error::from_parse_error(
				e,
				line,
				template_filename
					.into_os_string()
					.into_string()
					.unwrap_or_else(|_| "<Invalid Filename>".into()),
			);

			return Err(error);
		};
	}

	if let Err(e) = output_file.flush() {
		match e.kind() {
			std::io::ErrorKind::UnexpectedEof => {}
			_ => return Err(Error::CouldNotWrite(output_filename)),
		}
	}

	// Push the generated file is this scope to avoid cloning the path
	generated_files.push(output_filename);

	Ok(())
}

fn attatch_variables(
	files_variables: &mut HashMap<String, String>,
	cli_variables: Vec<(String, String)>,
	output_name: String,
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
	use std::fs;

	use crate::config::toml_config::TomlConfig;

	use super::*;

	#[test]
	fn should_attach_variables() {
		let mut files_variables = HashMap::new();
		files_variables.insert("namespace".into(), "app".into());
		let cli_variables = vec![];
		let output_name = "foo".into();

		attatch_variables(&mut files_variables, cli_variables, output_name);
		assert_eq!(files_variables["namespace"], "app");
		assert_eq!(files_variables["name"], "foo");
	}

	#[test]
	fn should_generate_file() {
		let cli_variables = vec![];
		let template_name = "foo".into();
		let output = PathBuf::from("./files/output/bar");

		let mut variables = HashMap::new();

		variables.insert("namespace".into(), "app".into());

		let config = Config {
			toml_config: TomlConfig {
				templates_path: PathBuf::from("./files/templates/"),
				variables,
			},
		};

		let _ = fs::create_dir_all("./files/templates/foo");
		fs::write("./files/templates/foo/foo", "{(namespace)}").unwrap();
		let _ = fs::remove_dir_all("./files/templates/foo/foo");
		generate(cli_variables, template_name, output, config).unwrap();

		let contents = fs::read_to_string("./files/output/foo").unwrap();
		assert_eq!(contents, "app");
	}

	#[test]
	fn should_validate_template_is_dir() {
		let template_name = "temp2".into();
		let output = PathBuf::from("./files/output/bar2");

		let mut variables = HashMap::new();
		variables.insert("namespace".into(), "app".into());

		let config = Config {
			toml_config: TomlConfig {
				templates_path: PathBuf::from("./files/templates/"),
				variables,
			},
		};

		let _ = fs::create_dir_all("./files/templates/");
		fs::write("./files/templates/temp2", "").unwrap();

		let error = generate(vec![], template_name, output, config).unwrap_err();
		assert!(
			matches!(error, Error::TemplateNotValid { .. }),
			"Error generated was: {error:?}"
		);
	}

	#[test]
	fn should_parse_filenames() {
		let cli_variables = vec![];
		let template_name = "filename_template".into();
		let output = PathBuf::from("./files/output/myoutput");

		let variables = HashMap::new();

		let config = Config {
			toml_config: TomlConfig {
				templates_path: PathBuf::from("./files/templates/"),
				variables,
			},
		};

		let _ = fs::create_dir_all("./files/templates/filename_template");

		fs::write(
			"./files/templates/filename_template/{(name)}.txt",
			"message",
		)
		.expect("Generate filename_template");

		generate(cli_variables, template_name, output, config).unwrap();
		let contents =
			fs::read_to_string("./files/output/myoutput.txt").expect("Reading myoutput file");
		assert_eq!(contents, "message");
	}

	#[test]
	fn should_generate_recursively() {
		let config = Config {
			toml_config: TomlConfig {
				templates_path: PathBuf::from("./files/templates/"),
				variables: HashMap::new(),
			},
		};
		let cli_variables = vec![];

		fs::create_dir_all("./files/templates/parent_dir").expect("Creating parent_dir");
		fs::create_dir_all("./files/templates/parent_dir/child_dir").expect("Creating child_dir");

		fs::write("./files/templates/parent_dir/{(name)}.txt", "message")
			.expect("Generating child_dir file");
		fs::write(
			"./files/templates/parent_dir/child_dir/{(name)}.txt",
			"message",
		)
		.expect("Generating child_dir file");

		generate(
			cli_variables,
			"parent_dir".to_owned(),
			PathBuf::from("./files/output/item1"),
			config,
		)
		.unwrap();

		let parent_content =
			fs::read_to_string("./files/output/item1.txt").expect("Parent file content not match");
		assert_eq!(parent_content, "message");

		let child_content = fs::read_to_string("./files/output/child_dir/item1.txt")
			.expect("Child file content not match");
		assert_eq!(child_content, "message");
	}
}
