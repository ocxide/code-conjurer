use std::path::PathBuf;

use once_cell::sync::Lazy;

pub fn get_ext(filename: &str) -> Option<&str> {
	let pos = filename.rfind('.')?;
	Some(&filename[pos + 1..])
}

static TEMPLATE_DIR: Lazy<PathBuf> =
	Lazy::new(|| PathBuf::try_from("./files/templates/").expect("Template Directory path invalid"));

pub fn get_template_path(filename: &str) -> PathBuf {
	let mut path = TEMPLATE_DIR.clone();
	path.push(filename);
	path
}
