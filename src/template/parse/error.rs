pub enum Error {
	Internal(InternalError),
	External(std::io::Error),
}

#[derive(Debug)]
pub enum InternalError {
	ParamNotFound(ParamNotFound),
	PipeNotFound(PipeUndefined),
}

#[derive(Debug)]
pub struct ParamNotFound {
	pub start: usize,
	pub end: usize,
}

#[derive(Debug)]
pub struct PipeUndefined {
	pub pipename: String,
	pub slice: (usize, usize),
}

impl PipeUndefined {
	pub fn new(pipename: String, slice: (usize, usize)) -> Self {
		Self { pipename, slice }
	}
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

impl From<PipeUndefined> for Error {
	fn from(err: PipeUndefined) -> Self {
		Self::Internal(InternalError::PipeNotFound(err))
	}
}

impl From<PipeUndefined> for InternalError {
	fn from(err: PipeUndefined) -> Self {
		InternalError::PipeNotFound(err)
	}
}

impl From<ParamNotFound> for InternalError {
	fn from(err: ParamNotFound) -> Self {
		InternalError::ParamNotFound(err)
	}
}
