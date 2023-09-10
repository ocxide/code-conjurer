use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum TomlConfigError {
	#[error(transparent)]
	NotFoundIn(#[from] NotFoundIn),

	#[error("Toml config unparseable: \n{0}")]
	Unparseable(#[from] toml::de::Error),

    #[error("Toml config missing field: {0}")]
    MissingField(&'static str),
}

#[derive(Debug)]
pub struct NotFoundIn(pub Box<[PathBuf]>);
impl std::error::Error for NotFoundIn {}
impl std::fmt::Display for NotFoundIn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut message = "Toml config not found in:\n".to_string();
		self
			.0
			.iter()
			.map(|path| {
				let mut displayed = path.display().to_string();
				displayed.push_str(" or\n");
				displayed
			})
			.for_each(|line| message.push_str(&line));

		message.pop(); // \n
		message.pop(); // r
		message.pop(); // o
		message.pop(); // %20

		f.write_str(&message)
	}
}
