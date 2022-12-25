use std::io::{self, Stdout};

use crossterm::{
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

pub fn run_app<F>(fun: F) -> io::Result<()>
where
	F: FnOnce(&mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()>,
{
	enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	// create app and run it
	fun(&mut terminal)?;

	// restore terminal
	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

	Ok(())
}
