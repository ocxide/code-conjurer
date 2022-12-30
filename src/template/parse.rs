use std::{
	iter::{Enumerate, Skip},
	str::Chars,
};

use std::collections::HashMap;

use super::pipes::{capitalize_once, capitalize_all};

type Pipe = Box<dyn (Fn(&str) -> String) + 'static>;
type PipesMap = HashMap<String, Pipe>;

#[derive(Debug)]
pub struct ParamNotFound<'a> {
	pub template: &'a str,
	pub start: usize,
	pub end: usize,
}

const SEPARATOR: char = '|';

pub fn parse<'a>(
	template: &'a str,
	params: &HashMap<String, String>,
) -> Result<String, ParamNotFound<'a>> {
	let mut pipes: PipesMap = HashMap::new();
	pipes.insert("capitalize_once".into(), Box::new(capitalize_once));
	pipes.insert("capitalize_all".into(), Box::new(|slice| capitalize_all(slice, '-')));

	_parse(template, params, pipes)
}

fn _parse<'a>(
	template: &'a str,
	params: &HashMap<String, String>,
	pipes: PipesMap,
) -> Result<String, ParamNotFound<'a>> {
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

		let value = params.get(var_name).ok_or(ParamNotFound {
			template,
			end,
			start,
		})?;

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
