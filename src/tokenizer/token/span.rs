use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenSpan {
	pub line: usize,
	pub column: usize,
}

impl TokenSpan {
	pub const fn new(line: usize, column: usize) -> Self {
		Self { line, column }
	}
}

impl Display for TokenSpan {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.line, self.column)
	}
}

impl From<(usize, usize)> for TokenSpan {
	fn from((line, column): (usize, usize)) -> Self {
		Self { line, column }
	}
}
