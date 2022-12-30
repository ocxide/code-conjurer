use std::{path::{PathBuf, Path}, collections::HashMap};

use once_cell::sync::Lazy;

use crate::template::{ParamNotFound, parse};

pub fn get_ext(filename: &str) -> Option<&str> {
	let pos = filename.rfind('.')?;
	Some(&filename[pos + 1..])
}

static TEMPLATE_DIR: Lazy<PathBuf> = Lazy::new(|| {
	PathBuf::try_from(r#"D:\Code\rust\code-conjurer\files\templates\"#)
		.expect("Template Directory path invalid")
});

pub fn get_template_path(filename: &str) -> PathBuf {
	let mut path = TEMPLATE_DIR.clone();
	path.push(filename);
	path
}

pub fn parse_path<'a>(path: &'a Path, params: &HashMap<String, String>) -> Result<PathBuf, ParamNotFound<'a>> {
	let path = match path.to_str() {
		Some(s) => s,
		None => return Ok(path.to_owned()),
	};

	let parsed = parse(path, params)?;
	Ok(PathBuf::from(parsed))
}