use {
	crate::{record, Token},
	color_eyre::{eyre::Context, Result},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Tokenizer {
	pub(crate) input: Vec<char>,
	pub(crate) position: usize,
	pub(crate) read_position: usize,
	pub(crate) char: char,
}

/// public methods
impl Tokenizer {
	#[tracing::instrument(level = "TRACE")]
	pub fn new(input: Vec<char>) -> Self {
		let mut tokenizer = Self { input, position: 0, read_position: 0, char: '\0' };
		tokenizer.read_char();
		tokenizer
	}

	#[tracing::instrument(level = "DEBUG", skip(self), fields(char = %self.char), ret)]
	pub fn next_token(&mut self) -> Result<Token> {
		self.skip_whitespace();

		let token = match self.char {
			'=' if matches!(self.peek_char(), Some('=')) => {
				self.read_char();
				Token::Equal
			}
			'=' => Token::Assign,
			'+' => Token::Plus,
			'-' => Token::Minus,
			'!' if matches!(self.peek_char(), Some('=')) => {
				self.read_char();
				Token::NotEqual
			}
			'!' => Token::Bang,
			'*' => Token::Asterisk,
			'/' => Token::Slash,
			'<' => Token::LessThan,
			'>' => Token::GreaterThan,
			',' => Token::Comma,
			';' => Token::Semicolon,
			'(' => Token::LeftParen,
			')' => Token::RightParen,
			'{' => Token::LeftBrace,
			'}' => Token::RightBrace,
			'\0' => Token::Eof,
			_ if self.is_digit() => return self.read_integer(),
			_ if self.is_letter() => return Ok(self.read_identifier()),
			char => Token::illegal(char),
		};

		self.read_char();
		Ok(token)
	}
}

/// private methods
impl Tokenizer {
	#[tracing::instrument(
		level = "DEBUG",
		skip(self),
		fields(reading, at = %self.read_position)
	)]
	fn read_char(&mut self) {
		match record!("reading", self.input.get(self.read_position)) {
			// We either just started parsing or we are done.
			None => self.char = '\0',
			Some(&next) => self.char = next,
		};

		self.position = self.read_position;
		self.read_position += 1;
	}

	#[tracing::instrument(level = "DEBUG", skip(self), ret)]
	fn peek_char(&self) -> Option<char> {
		self.input.get(self.read_position).copied()
	}

	#[tracing::instrument(level = "DEBUG", skip(self), ret)]
	fn read_identifier(&mut self) -> Token {
		let position = self.position;

		while self.is_letter() {
			self.read_char();
		}

		let ident = String::from_iter(&self.input[position..self.position]);

		match ident.as_str() {
			"fn" => Token::Function,
			"let" => Token::Let,
			"if" => Token::If,
			"else" => Token::Else,
			"return" => Token::Return,
			"true" => Token::True,
			"false" => Token::False,
			_ => Token::Ident(ident),
		}
	}

	#[tracing::instrument(level = "DEBUG", skip(self), ret)]
	fn read_integer(&mut self) -> Result<Token> {
		let position = self.position;

		while self.is_digit() {
			self.read_char();
		}

		let int = String::from_iter(&self.input[position..self.position])
			.parse::<i64>()
			.context("Failed to parse integer")?;

		Ok(Token::Int(int))
	}

	#[tracing::instrument(level = "TRACE", skip(self))]
	fn skip_whitespace(&mut self) {
		while self.char.is_whitespace() {
			self.read_char();
		}
	}

	#[tracing::instrument(level = "TRACE", skip(self), fields(char = %self.char), ret)]
	fn is_letter(&self) -> bool {
		self.char.is_alphabetic() || self.char == '_'
	}

	#[tracing::instrument(level = "TRACE", skip(self), fields(char = %self.char), ret)]
	fn is_digit(&self) -> bool {
		self.char.is_digit(10)
	}
}
