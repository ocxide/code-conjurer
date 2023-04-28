use std::io;

use tui::widgets::{List, ListItem};

use crate::{
	config::Config,
	dir_browser::{
		browser::DirBrowser,
		entry::{Entry, Symlink},
	},
	terminal::utils::run_app,
};

pub fn list(config: &Config) -> io::Result<()> {
	let browser = DirBrowser::new(config.toml_config.templates_path.clone())?;
	run_app(|terminal| {
		terminal
			.draw(|frame| {
				let items: Vec<_> = browser
					.read_dir()
					.map(|entry| match entry {
						Entry::File(filename) => ListItem::new(filename.to_owned()),
						Entry::Directory(name) => ListItem::new(format!("📁 {name}")),
						Entry::Symlink(Symlink { name, .. }) => ListItem::new(format!("Link: {name}")),
					})
					.collect();

				let list = List::new(items);
				frame.render_widget(list, frame.size())
			})
			.map(|_| {})
	})
}
