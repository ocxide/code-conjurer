pub mod error;

use std::{
	io::Write,
	iter::{Enumerate, Skip},
	str::Chars,
};

use std::collections::HashMap;

use self::error::{Error, ParamNotFound};

use super::pipes::{capitalize_all, capitalize_once};

type Pipe = fn(&str) -> String;
type PipesMap = HashMap<String, Pipe>;

pub trait TemplateParse {
	/// Parses the template and writes the result to `writter`.
	/// Note that this method may write a lot of times, so ensure to use a BufWrite if working with
	/// blocking systems.
	fn parse<W: Write>(&self, content: &str, writter: &mut W) -> Result<(), Error>;
}

pub struct DefaultTemplateParse {
	pipes: PipesMap,
	vars: HashMap<String, String>,
}

impl DefaultTemplateParse {
	pub fn with_vars(vars: HashMap<String, String>) -> Self {
		let mut pipes: PipesMap = HashMap::new();
		pipes.insert("capitalize_once".to_owned(), capitalize_once);
		pipes.insert("capitalize_all".into(), |slice| capitalize_all(slice, '-'));

		Self { pipes, vars }
	}
}

impl TemplateParse for DefaultTemplateParse {
	fn parse<W: Write>(&self, content: &str, writter: &mut W) -> Result<(), Error> {
		let mut i = 0usize;

		for TemplateParam { name, start, end } in ParamsBrowser::new(content) {
			/* Previous slice */
			let back = &content[i..start];

			writter.write(back.as_bytes())?;

			let (var_name, pipes_iter) = {
				let mut var_slices = name.split(SEPARATOR);
				/* Slipt always return a first param */
				let var_name = var_slices.next().unwrap();
				let pipes = var_slices;

				(var_name, pipes)
			};

			let value = self
				.vars
				.get(var_name)
				.ok_or_else(|| ParamNotFound { end, start })?;

			let piped_value = apply_pipes(value, pipes_iter, &self.pipes);
			writter.write(piped_value.as_bytes())?;

			i = end;
		}

		writter.write(content[i..].as_bytes())?;
		Ok(())
	}
}

const SEPARATOR: char = '|';

pub fn parse<'a>(
	template: &'a str,
	params: &HashMap<String, String>,
) -> Result<String, ParamNotFound> {
	let mut pipes: PipesMap = HashMap::new();
	pipes.insert("capitalize_once".into(), capitalize_once);
	pipes.insert("capitalize_all".into(), |slice| capitalize_all(slice, '-'));

	_parse(template, params, pipes)
}

fn _parse(
	template: &str,
	params: &HashMap<String, String>,
	pipes: PipesMap,
) -> Result<String, ParamNotFound> {
	let mut parsed = String::new();
	let mut i = 0usize;

	for TemplateParam { name, start, end } in ParamsBrowser::new(template) {
		/* Previous slice */
		let back = &template[i..start];

		parsed.push_str(back);

		let (var_name, pipes_iter) = {
			let mut var_slices = name.split(SEPARATOR);
			/* Slipt always return a first param */
			let var_name = var_slices.next().unwrap();
			let pipes = var_slices;

			(var_name, pipes)
		};

		let value = params
			.get(var_name)
			.ok_or_else(|| ParamNotFound { end, start })?;

		let piped_value = apply_pipes(value, pipes_iter, &pipes);
		parsed.push_str(&piped_value);

		i = end;
	}

	parsed.push_str(&template[i..]);

	Ok(parsed)
}

static LEFT: &str = "{(";
static RIGHT: &str = ")}";

static LEFT_LEN: usize = LEFT.len();
static RIGHT_LEN: usize = RIGHT.len();

struct ParamsBrowser<'a> {
	template: &'a str,
	name_at: Option<usize>,
	iter: Skip<Enumerate<Chars<'a>>>,
}

impl<'a> ParamsBrowser<'a> {
	pub fn new(template: &'a str) -> Self {
		let iter = template.chars().enumerate().skip(LEFT_LEN - 1);

		Self {
			template,
			iter,
			name_at: None,
		}
	}
}

struct TemplateParam<'a> {
	name: &'a str,
	start: usize,
	end: usize,
}

impl<'a> Iterator for ParamsBrowser<'a> {
	type Item = TemplateParam<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		for (i, _) in self.iter.by_ref() {
			match &self.name_at {
				None => {
					let maybe = &self.template[(i - (LEFT_LEN - 1))..=i];
					if maybe == LEFT {
						self.name_at = Some(i + 1);
						continue;
					}
				}
				Some(at) => {
					let maybe = &self.template[(i - (RIGHT_LEN - 1))..=i];
					let found = maybe == RIGHT;

					if found {
						let end = i - 1;
						let name = &self.template[*at..end];

						let param = TemplateParam {
							name,
							start: *at - LEFT_LEN,
							end: i + 1,
						};
						self.name_at = None;

						return Some(param);
					}
				}
			}
		}

		None
	}
}

fn apply_pipes<'a>(
	value: &'a str,
	split: impl Iterator<Item = &'a str>,
	pipes: &PipesMap,
) -> String {
	let mut pipes_fns = split.filter_map(|pipe_name| pipes.get(pipe_name));
	let first_pipe = match pipes_fns.next() {
		Some(first_pipe) => first_pipe,
		None => return value.to_owned(),
	};

	let mut result = first_pipe(value);

	for pipe in pipes_fns {
		result = pipe(&result);
	}

	// Return the `result` variable.
	result
}
