use std::collections::HashMap;
use std::{
	iter::{Enumerate, Skip},
	str::Chars,
};

#[derive(Debug)]
pub struct ParamNotFound;

pub fn parse(template: &str, params: &HashMap<String, String>) -> Result<String, ParamNotFound> {
	let mut parsed = String::new();
	let mut i = 0usize;

	for TemplateParam { name, start, end } in ParamsBrowser::new(template) {
		let slice = &template[i..=start];

		parsed.push_str(slice);
		let value = params.get(name).ok_or(ParamNotFound)?;
		parsed.push_str(value);

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
							start: *at - 3,
							end: i+1,
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
