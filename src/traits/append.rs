use std::{collections::HashMap, hash::Hash};

pub trait Append {
	fn append(&mut self, appended: Self) -> &mut Self;
}

impl<K: Eq + Hash, U> Append for HashMap<K, U> {
	fn append(&mut self, appended: Self) -> &mut Self {
		for (key, value) in appended {
			self.insert(key, value);
		}
		self
	}
}
