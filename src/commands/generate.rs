use std::{collections::HashMap, fs};

use miette::NamedSource;

use crate::{
	cli::GenerateCommand,
	diagnostics::{
		file_not_found::FileNotFoundDiagnostic, filename_invalid::FilenameInvalid,
		param_not_found::ParamNotFoundDiagnostic,
	},
	template::parse,
};

pub fn generate(command: GenerateCommand) -> miette::Result<()> {
	let GenerateCommand {
		output,
		params,
		template,
	} = command;

	/* Get file name and extension from output file */
	let (name, _) = filename_from_path(&output)
		.ok_or_else(|| FilenameInvalid::new(NamedSource::new(&output, "")))?;

	/* Transform clap params into HashMap */
	let params_map = into_params(params, name);

	/* Read template content */
	let template_content =
		fs::read_to_string(&template).map_err(|_| FileNotFoundDiagnostic::from_path(&template))?;

	/* Parse over lines to avoid entire file content duplication */
	let parsed = template_content
		.lines()
		.map(|line| {
			parse(line, &params_map).map_err(|e| ParamNotFoundDiagnostic::from_error(e, &template))
		})
		.collect::<Result<String, ParamNotFoundDiagnostic>>()?;

	print!("{}", parsed);

	Ok(())
}

fn into_params(params: Vec<(String, String)>, name: &str) -> HashMap<String, String> {
	let mut map = HashMap::new();
	map.insert("name".into(), name.into());
	for (key, value) in params {
		map.insert(key, value);
	}

	map
}

fn filename_from_path(path: &str) -> Option<(&str, &str)> {
	let pos = path.rfind('/');
	let filename = match pos {
		Some(pos) => &path[pos + 1..],
		None => path,
	};

	let dot_pos = filename.find('.')?;
	let ext = &filename[dot_pos..];
	let name = &filename[0..dot_pos];

	Some((name, ext))
}
