pub trait Ignore {
	fn ignore(&self) {}
}

impl<T, E> Ignore for Result<T, E> {}

impl<T> Ignore for Option<T> {}
