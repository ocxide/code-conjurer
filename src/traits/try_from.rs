pub trait MyTryFrom<T>: Sized {
	type Error;

	fn my_try_from(from: T) -> Result<Self, Self::Error>;
}

pub trait MyTryInto<T>: Sized {
	type Error;

	fn my_try_into(self) -> Result<T, Self::Error>;
}

impl<T, U> MyTryInto<U> for T
where
	U: MyTryFrom<T>,
{
	type Error = U::Error;

	fn my_try_into(self) -> Result<U, U::Error> {
		U::my_try_from(self)
	}
}
