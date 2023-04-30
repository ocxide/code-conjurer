use crate::{
	config::Config,
	dir_browser::{
		browser::DirBrowser,
		entry::{Entry, Symlink},
	},
};
use std::io;

pub fn list(config: &Config) -> io::Result<()> {
	let browser = DirBrowser::new(config.toml_config.templates_path.clone())?;

	let list: String = browser
		.into_iter()
		.map(|entry| match entry {
			Entry::File(filename) => filename,
			Entry::Directory(name) => format!("📁 {name}"),
			Entry::Symlink(Symlink { name, .. }) => format!("Link: {name}"),
		})
		.map(|mut line| {
			line.push('\n');
			line
		})
		.collect();

	println!("{list}");

	Ok(())
}
