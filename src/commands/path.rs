use std::env::current_exe;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
#[error("Could not get exe file path")]
struct PrintError;

pub fn print_path() -> miette::Result<()> {
	match current_exe() {
		Ok(path) => Ok(println!("{:?}", path)),
		Err(_) => Err(PrintError.into()),
	}
}
