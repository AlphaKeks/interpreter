use {
	color_eyre::{eyre::Context, Result},
	std::{iter::Peekable, str::Chars},
	token::{TokenKind, TokenSpan, KEYWORDS},
};

#[cfg(test)]
mod tests;

pub mod token;
pub use token::Token;

#[derive(Debug, Clone)]
pub struct Tokenizer<'input> {
	/// The original input
	input: &'input str,

	/// An iterator over the input
	iter: Peekable<Chars<'input>>,

	/// Current position in the input
	position: usize,

	/// Current line
	line: usize,

	/// Current column
	column: usize,
}

impl<'input> Tokenizer<'input> {
	pub fn new(input: &'input str) -> Tokenizer<'input> {
		Self { input, iter: input.chars().peekable(), position: 0, line: 1, column: 0 }
	}
}

impl<'input> Iterator for Tokenizer<'input> {
	type Item = Result<Token>;

	/// The main driver of [`Tokenizer`].
	///
	/// This advances it token by token until the input is exhausted.
	fn next(&mut self) -> Option<Self::Item> {
		self.skip_whitespace();

		let next = self.advance()?;
		let line = self.line;
		let column = self.column;
		let peek_is_equal_sign = matches!(self.iter.peek(), Some('='));

		let token = self.token(match next {
			'=' if peek_is_equal_sign => {
				self.advance();
				let token = self.token_at(TokenKind::Equal, line, column);
				return Some(Ok(token));
			}
			'=' => TokenKind::Assign,
			'+' => TokenKind::Plus,
			'-' => TokenKind::Minus,
			'/' => TokenKind::Slash,
			'*' => TokenKind::Asterisk,
			'%' => TokenKind::Modulo,
			'!' if peek_is_equal_sign => {
				self.advance();
				let token = self.token_at(TokenKind::NotEqual, line, column);
				return Some(Ok(token));
			}
			'!' => TokenKind::Bang,
			'<' if peek_is_equal_sign => {
				self.advance();
				let token = self.token_at(TokenKind::LessThanOrEqual, line, column);
				return Some(Ok(token));
			}
			'<' => TokenKind::LessThan,
			'>' if peek_is_equal_sign => {
				self.advance();
				let token = self.token_at(TokenKind::GreatherThanOrEqual, line, column);
				return Some(Ok(token));
			}
			'>' => TokenKind::GreatherThan,
			',' => TokenKind::Comma,
			';' => TokenKind::Semicolon,
			'(' => TokenKind::LeftParen,
			')' => TokenKind::RightParen,
			'{' => TokenKind::LeftBrace,
			'}' => TokenKind::RightBrace,

			input if input.is_ascii_alphabetic() || input == '_' => {
				let identifier = self.read_identifier();
				let token = self.token_at(identifier, line, column);
				return Some(Ok(token));
			}

			input if input.is_ascii_digit() => match self.read_integer() {
				Ok(integer) => {
					let token = self.token_at(integer, line, column);
					return Some(Ok(token));
				}
				Err(err) => return Some(Err(err)),
			},

			illegal => TokenKind::Illegal(illegal),
		});

		Some(Ok(token))
	}
}

impl<'input> Tokenizer<'input> {
	/// Advances `self.iter` and updates `self.line` and `self.column`.
	fn advance(&mut self) -> Option<char> {
		let next = self.iter.next()?;
		self.position += next.len_utf8();
		self.column += 1;
		if next == '\n' {
			self.line += 1;
			self.column = 0;
		}

		Some(next)
	}

	/// Advances `self.iter` until it reads a non-whitespace character.
	fn skip_whitespace(&mut self) {
		while matches!(self.iter.peek(), Some(c) if c.is_whitespace()) {
			self.advance();
		}
	}

	/// Advances `self.iter` while reading an identifier and returns it.
	fn read_identifier(&mut self) -> TokenKind {
		let identifier = self.read_while(|c| c.is_ascii_alphabetic() || c == '_');

		match KEYWORDS.get(identifier) {
			None => identifier.into(),
			Some(keyword) => keyword.to_owned(),
		}
	}

	/// Advances the [`Tokenizer`] while reading an integer and returns it.
	fn read_integer(&mut self) -> Result<TokenKind> {
		self.read_while(|c| c.is_ascii_digit())
			.parse::<i64>()
			.context("Failed to parse integer")
			.map(Into::into)
	}

	/// Advances [`Tokenizer`] while the given `predicate` is met. Returns the chunk that was read.
	fn read_while(&mut self, predicate: impl Fn(char) -> bool) -> &str {
		let start = self.position - 1;

		while matches!(self.iter.peek(), Some(&c) if predicate(c)) {
			self.advance();
		}

		&self.input[start..self.position]
	}

	/// Constructs a [`Token`] from the given input and current line / column.
	fn token(&self, kind: impl Into<TokenKind>) -> Token {
		Token::new(kind.into(), TokenSpan::new(self.line, self.column))
	}

	/// Constructs a [`Token`] from the given input at the given current line / column.
	fn token_at(&self, kind: impl Into<TokenKind>, line: usize, column: usize) -> Token {
		Token::new(kind.into(), TokenSpan::new(line, column))
	}
}
