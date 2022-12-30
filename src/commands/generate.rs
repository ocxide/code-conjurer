use std::{
	collections::HashMap,
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use miette::IntoDiagnostic;

use crate::{
	cli::GenerateCommand,
	diagnostics::{file_not_found::FileNotFoundDiagnostic, param_not_found::ParamNotFoundDiagnostic},
	path::{get_ext, get_template_path},
	template::parse,
	traits::into_miette::IntoMiette,
};

pub fn generate(command: GenerateCommand, mut output: PathBuf) -> miette::Result<()> {
	let GenerateCommand { params, template } = command;

	let output_name = output.file_name().into_miette("Output")?;

	let template_path = get_template_path(&template);
	let template_ext = get_ext(&template).unwrap_or("");
	/* Transform clap params into HashMap */
	let params_map = into_params(params, &output_name);

	/* Read template content */
	let template_content =
		fs::read_to_string(&template_path).map_err(|_| FileNotFoundDiagnostic::new(&template))?;

	/* Parse over lines to avoid entire file content duplication */
	let parsed = template_content
		.lines()
		.map(|line| {
			let mut line =
				parse(line, &params_map).map_err(|e| ParamNotFoundDiagnostic::from_error(e, &template))?;
			line.push('\n');
			Ok(line)
		})
		.collect::<Result<String, ParamNotFoundDiagnostic>>()?;

	output.set_extension(template_ext);
	let mut output_file = File::create(&output).into_miette((&output, "Output"))?;

	output_file.write_all(parsed.as_bytes()).into_diagnostic()?;

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
