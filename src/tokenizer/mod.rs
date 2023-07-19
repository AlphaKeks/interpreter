use {
	crate::{record, Token},
	color_eyre::{eyre::Context, Result},
};

#[cfg(test)]
mod tests;

pub struct Tokenizer {
	pub(crate) input: Vec<char>,
	pub(crate) position: usize,
	pub(crate) read_position: usize,
	pub(crate) char: char,
}

/// public methods
impl Tokenizer {
	#[tracing::instrument(level = "TRACE", ret)]
	pub fn new(input: Vec<char>) -> Self {
		let mut tokenizer = Self { input, position: 0, read_position: 0, char: '\0' };
		tokenizer.next_char();
		tokenizer
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	pub fn step(&mut self) -> Result<Token> {
		self.skip_whitespace();

		let token = match self.char {
			'=' if matches!(self.peek_char(), Some('=')) => {
				self.next_char();
				Token::Equal
			}
			'=' => Token::Assign,
			'+' => Token::Plus,
			'-' => Token::Minus,
			'!' if matches!(self.peek_char(), Some('=')) => {
				self.next_char();
				Token::NotEqual
			}
			'!' => Token::Bang,
			'*' => Token::Asterisk,
			'/' => Token::Slash,
			'<' => Token::LessThan,
			'>' => Token::GreaterThan,
			',' => Token::Comma,
			':' => Token::Colon,
			';' => Token::Semicolon,
			'(' => Token::LeftParen,
			')' => Token::RightParen,
			'{' => Token::LeftBrace,
			'}' => Token::RightBrace,
			'[' => Token::LeftBracket,
			']' => Token::RightBracket,
			'\0' => Token::Eof,
			'"' => self.read_string(),
			_ if self.is_digit() => return self.read_integer(),
			_ if self.is_letter() => return Ok(self.read_identifier()),
			char => Token::illegal(char),
		};

		self.next_char();
		Ok(token)
	}
}

/// private methods
impl Tokenizer {
	#[tracing::instrument(level = "DEBUG", fields(reading))]
	fn next_char(&mut self) {
		match record!("reading", self.input.get(self.read_position)) {
			// We either just started parsing or we are done.
			None => self.char = '\0',
			Some(&next) => self.char = next,
		};

		self.position = self.read_position;
		self.read_position += 1;
	}

	#[tracing::instrument(level = "TRACE", ret)]
	fn peek_char(&self) -> Option<char> {
		self.input.get(self.read_position).copied()
	}

	#[tracing::instrument(level = "TRACE", ret)]
	fn read_identifier(&mut self) -> Token {
		let position = self.position;

		while self.is_letter() {
			self.next_char();
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

	#[tracing::instrument(level = "TRACE", ret)]
	fn read_string(&mut self) -> Token {
		let position = self.position + 1;

		loop {
			self.next_char();
			if matches!(self.char, '"' | '\0') {
				break;
			}
		}

		Token::String(String::from_iter(&self.input[position..self.position]))
	}

	#[tracing::instrument(level = "TRACE", ret)]
	fn read_integer(&mut self) -> Result<Token> {
		let position = self.position;

		while self.is_digit() {
			self.next_char();
		}

		let int = String::from_iter(&self.input[position..self.position])
			.parse::<i64>()
			.context("Failed to parse integer")?;

		Ok(Token::Int(int))
	}

	#[tracing::instrument(level = "TRACE")]
	fn skip_whitespace(&mut self) {
		while self.char.is_whitespace() {
			self.next_char();
		}
	}

	#[tracing::instrument(level = "TRACE", ret)]
	fn is_letter(&self) -> bool {
		self.char.is_alphabetic() || self.char == '_'
	}

	#[tracing::instrument(level = "TRACE", ret)]
	fn is_digit(&self) -> bool {
		self.char.is_digit(10)
	}
}

impl std::fmt::Debug for Tokenizer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Tokenizer { position, read_position, char, .. } = self;
		write!(f, "{{ position: {position}, read_position: {read_position}, char: `{char}` }}")
	}
}
