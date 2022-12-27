use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
	backend::Backend,
	layout::{Constraint, Direction, Layout},
	style::{Modifier, Style},
	widgets::{List, ListItem, ListState},
	Frame,
};

use crate::{
	dir_browser::{
		browser::DirBrowser,
		entry::{Entry, Symlink},
	},
	traits::{ignore::Ignore, try_default::TryDefault},
};

use super::utils::run_app;

struct DirState {
	browser: DirBrowser,
	selected: ListState,
}

impl TryDefault for DirState {
	type Error = io::Error;

	fn try_default() -> Result<Self, Self::Error> {
		Ok(DirState {
			browser: DirBrowser::try_default()?,
			selected: ListState::default(),
		})
	}
}

pub fn create_file() -> io::Result<String> {
	let mut state = DirState::try_default()?;

	run_app(|terminal| {
		loop {
			terminal.draw(|f| frame(f, &mut state))?;
			if let Event::Key(key) = event::read()? {
				match key.code {
					KeyCode::Esc => return Err(io::ErrorKind::Interrupted.into()),
					KeyCode::Left => state.browser.back().ignore(),
					KeyCode::Down => {
						if let Some(selected) = state.selected.selected() {
							state.selected.select(Some(selected + 1));
						} else {
							state.selected.select(Some(0));
						}
					}
					KeyCode::Up => {
						if let Some(selected) = state.selected.selected() {
							if selected > 0 {
								state.selected.select(Some(selected - 1));
							}
						} else {
							state.selected.select(Some(0));
						}
					}
					KeyCode::Enter => break,
					_ => todo!(),
				}
			};
		}

		Ok(())
	})?;

	Ok("".to_string())
}

fn frame<B: Backend>(frame: &mut Frame<B>, state: &mut DirState) {
	let win = frame.size();
	let layouts = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Percentage(70), Constraint::Length(20)])
		.split(win);

	let up_layout = layouts[0];
	let down_layout = layouts[1];

	let items: Vec<_> = state
		.browser
		.read_dir()
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
	frame.render_stateful_widget(list, up_layout, &mut state.selected);
}
