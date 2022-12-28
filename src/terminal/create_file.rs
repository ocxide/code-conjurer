use crossterm::event::{self, Event, KeyCode};
use std::io;
use std::path::PathBuf;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout},
	style::{Modifier, Style},
	widgets::{List, ListItem, ListState, Paragraph},
	Frame,
};

use crate::{
	dir_browser::{
		browser::DirBrowser,
		entry::{Entry, Symlink},
	},
	traits::try_default::TryDefault,
};

use super::utils::run_app;

pub fn create_file() -> io::Result<PathBuf> {
	let mut browser = DirBrowser::try_default()?;
	let mut selected = ListState::default();
	let mut filtered = browser.read_dir().cloned().collect::<Vec<_>>();
	let mut query = String::new();

	run_app(|terminal| {
		loop {
			terminal.draw(|f| frame(f, &filtered, &mut selected, query.clone()))?;
			if let Event::Key(key) = event::read()? {
				match key.code {
					KeyCode::Esc => return Err(io::ErrorKind::Interrupted.into()),
					KeyCode::Left => {
						if browser.back().is_ok() {
							query = String::new();
							filtered = browser.read_dir().cloned().collect::<Vec<_>>();
						}
					}
					KeyCode::Right => {
						if let Some(s) = selected.selected() {
							if browser.enter(s).is_ok() {
								selected.select(Some(0));
								query = String::new();
								filtered = browser.read_dir().cloned().collect::<Vec<_>>();
							}
						}
					}
					KeyCode::Down => {
						if let Some(s) = selected.selected() {
							if s < browser.read_dir().count() - 1 {
								selected.select(Some(s + 1));
							}
						} else {
							selected.select(Some(0));
						}
					}
					KeyCode::Up => {
						if let Some(s) = selected.selected() {
							if s > 0 {
								selected.select(Some(s - 1));
							}
						} else {
							selected.select(Some(0));
						}
					}
					KeyCode::Char(c) => {
						query.push(c);
						filtered = filter_entries(browser.read_dir(), &query);
						selected.select(None);
					}
					KeyCode::Backspace => {
						if query.pop().is_some() {
							filtered = filter_entries(browser.read_dir(), &query);
							selected.select(None);
						}
					}
					KeyCode::Enter => break,
					_ => todo!(),
				}
			};
		}

		Ok(())
	})?;

	let file_path = browser.get_path().join(query);
	Ok(file_path)
}

fn frame<B: Backend>(
	frame: &mut Frame<B>,
	files: &[Entry],
	selected: &mut ListState,
	query: String,
) {
	let win = frame.size();
	let layouts = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Percentage(70), Constraint::Length(20)])
		.split(win);

	let up_layout = layouts[0];
	let down_layout = layouts[1];

	let items: Vec<_> = files
		.iter()
		.map(|entry| match entry {
			Entry::File(filename) => ListItem::new(filename.to_owned()),
			Entry::Directory(name) => ListItem::new(format!("📁 {name}")),
			Entry::Symlink(Symlink { name, .. }) => ListItem::new(format!("Link: {name}")),
		})
		.collect();

	let list = List::new(items)
		.highlight_symbol(">> ")
		.highlight_style(Style::default().add_modifier(Modifier::UNDERLINED));

	// frame.render_widget(list, up_layout);
	frame.render_stateful_widget(list, up_layout, selected);

	let query_p = Paragraph::new(query);
	frame.render_widget(query_p, down_layout);
}

fn filter_entries<'a>(entries: impl Iterator<Item = &'a Entry>, filter: &str) -> Vec<Entry> {
	entries
		.filter(|entry| {
			let name = match entry {
				Entry::File(filename) => filename,
				Entry::Directory(name) => name,
				Entry::Symlink(Symlink { name, .. }) => name,
			};

			name.contains(filter)
		})
		.cloned()
		.collect()
}
