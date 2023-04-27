use std::{
	collections::HashMap,
	fs,
	io::Write,
	path::{Path, PathBuf},
};

use miette::IntoDiagnostic;

use crate::{
	diagnostics::{file_not_found::FileNotFoundDiagnostic, param_not_found::ParamNotFoundDiagnostic},
	dir_browser::{browser::DirBrowser, entry::Entry, file_creator::create_file},
	path::{get_ext, parse_path},
	template::parse::parse,
	traits::{ignore::Ignore, into_miette::IntoMiette, try_from::MyTryInto}, config::Config,
};

fn into_params(params: Vec<(String, String)>, name: &str) -> HashMap<String, String> {
	let mut map = HashMap::new();
	map.insert("name".into(), name.into());
	for (key, value) in params {
		map.insert(key, value);
	}

	map
}

pub fn recursive_generate(
	params: Vec<(String, String)>,
	template_name: String,
	output: PathBuf,
    config: &Config,
) -> miette::Result<()> {
	let template_path = config.templates_path.join(&template_name);
	let output_name = output.file_name().into_miette("Output")?;
	let params = into_params(params, &output_name);

	let entry: Entry = template_path.clone().my_try_into().into_diagnostic()?;
	if let Entry::File(_) = entry {
		generate_file(&template_name, &template_path, output, &params)?;
		return Ok(());
	}

	let mut browser =
		DirBrowser::new(template_path.clone()).into_miette((&template_path, "Template"))?;
	let entries = browser.read_dir().cloned().collect::<Vec<_>>();

	/* Generate files into output dir not into the ouput itself */
	let output_dir = output.parent().unwrap_or(output.as_path());
	for (i, entry) in entries.iter().enumerate() {
		generate_entry(entry, &mut browser, output_dir, &params, i)?;
	}

	Ok(())
}

pub fn generate_entry(
	entry: &Entry,
	browser: &mut DirBrowser,
	output_path: &Path,
	params: &HashMap<String, String>,
	i: usize,
) -> miette::Result<()> {
	match entry {
		Entry::File(filename) => {
			let template_path = browser.get_path().join(filename);
			let output_path = output_path.join(filename);
			generate_file(filename, &template_path, output_path, params)?;
		}
		Entry::Directory(dirname) => generate_dir(i, dirname, browser, output_path, params)?,
		_ => todo!(),
	}

	Ok(())
}

fn generate_file(
	template_filename: &str,
	template_path: &PathBuf,
	mut output_path: PathBuf,
	params: &HashMap<String, String>,
) -> miette::Result<()> {
	let template_ext = get_ext(template_filename).unwrap_or("");

	let template_content = fs::read_to_string(template_path)
		.map_err(|_| FileNotFoundDiagnostic::new(template_filename))?;

	let parsed = template_content
		.lines()
		.map(|line| {
			let mut line = parse(line, params)
				.map_err(|e| ParamNotFoundDiagnostic::from_error(e, template_filename))?;
			line.push('\n');
			Ok(line)
		})
		.collect::<Result<String, ParamNotFoundDiagnostic>>()?;

	output_path = parse_path(&output_path, params)
		.map_err(|e| ParamNotFoundDiagnostic::from_error(e, template_filename))?;

	output_path.set_extension(template_ext);
	let mut output_file = create_file(&output_path).into_miette((&output_path, "Output"))?;
	output_file.write_all(parsed.as_bytes()).ignore();

	Ok(())
}

fn generate_dir(
	i: usize,
	dirname: &str,
	browser: &mut DirBrowser,
	output_path: &Path,
	params: &HashMap<String, String>,
) -> miette::Result<()> {
	/* Generate dir in output dir */
	let output_path = output_path.join(dirname);

	/* Enter into the template dir */
	browser.enter(i).into_diagnostic()?;

	/* Read template entries */
	let entries = browser
		.read_dir()
		.take_while(|entry| !matches!(entry, Entry::Directory(_)))
		.cloned()
		.collect::<Vec<_>>();

	for (i, entry) in entries.iter().enumerate() {
		generate_entry(entry, browser, &output_path, params, i)?;
	}

	/* Get back of template dir */
	browser.back().into_diagnostic()?;

	Ok(())
}
