pub enum Error {
	Internal(InternalError),
	External(std::io::Error),
}

#[derive(Debug)]
pub enum InternalError {
	ParamNotFound(ParamNotFound),
	PipeNotFound(&'static str),
}

#[derive(Debug)]
pub struct ParamNotFound {
	pub start: usize,
	pub end: usize,
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::External(err)
	}
}

impl From<InternalError> for Error {
	fn from(err: InternalError) -> Self {
		Self::Internal(err)
	}
}

impl From<ParamNotFound> for Error {
	fn from(err: ParamNotFound) -> Self {
		Self::Internal(InternalError::ParamNotFound(err))
	}
}
