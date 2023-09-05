use std::{
	collections::HashMap,
	path::{Path, PathBuf},
};

use crate::template::parse::{error::ParamNotFound, parse};

pub fn get_ext(filename: &str) -> Option<&str> {
	let pos = filename.rfind('.')?;
	Some(&filename[pos + 1..])
}

pub fn parse_path(path: &Path, params: &HashMap<String, String>) -> Result<PathBuf, ParamNotFound> {
	let path = match path.to_str() {
		Some(s) => s,
		None => return Ok(path.to_owned()),
	};

	let parsed = parse(path, params)?;
	Ok(PathBuf::from(parsed))
}
