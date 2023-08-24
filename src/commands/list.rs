use crate::{
	config::Config,
	dir_browser::{
		browser::DirBrowser,
		entry::{Entry, Symlink},
	},
};
use std::io::{self, stdout, IsTerminal, Stdout, Write};

pub fn list(config: &Config) -> io::Result<()> {
	let browser = DirBrowser::new(config.toml_config.templates_path.clone())?;

    let mut stdout = stdout();
    let fun = if stdout.is_terminal() {
        |stdout: &mut Stdout, entry: Entry| {
        match entry {
			Entry::File(filename) => writeln!(stdout, "{filename}"),
			Entry::Directory(name) => writeln!(stdout, "{name}"),
			Entry::Symlink(Symlink { name, .. }) => writeln!(stdout, "{name}"),
        }
        }
    } else {
        |stdout: &mut Stdout, entry: Entry| match entry {
			Entry::File(filename) => writeln!(stdout, "{filename}"),
			Entry::Directory(name) => writeln!(stdout, "📁 {name}"),
			Entry::Symlink(Symlink { name, .. }) => writeln!(stdout, "🔗 {name}"),
		}
    };

    browser.into_iter().map(|entry| fun(&mut stdout, entry)).collect()
}
