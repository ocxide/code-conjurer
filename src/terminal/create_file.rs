use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{backend::Backend, Frame};

use super::utils::run_app;

pub fn create_file() -> io::Result<String> {
	run_app(|terminal| {
		loop {
			terminal.draw(|f| frame(f))?;
			if let Event::Key(key) = event::read()? {
				match key.code {
					KeyCode::Esc => break,
					_ => todo!(),
				}
			};
		}

		Ok(())
	})?;

	Ok("".to_string())
}

fn frame<B: Backend>(frame: &mut Frame<B>) {}
