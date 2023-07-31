use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
	/// Any illegal (unexpected) characters we encounter
	Illegal(char),

	/// Keywords, variable names, etc.
	Identifier(String),

	/// Integers
	Integer(i64),

	/* Operators */
	Assign,
	Plus,
	Minus,
	Slash,
	Asterisk,
	Modulo,
	Bang,
	Equal,
	NotEqual,
	LessThan,
	GreatherThan,
	LessThanOrEqual,
	GreatherThanOrEqual,

	/* Delimiters */
	Comma,
	Semicolon,

	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,

	/* Keywords */
	Let,
	Function,
	Return,
	If,
	Else,
	True,
	False,
}

impl Display for TokenKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenKind::Illegal(illegal) => write!(f, "`{illegal}`"),
			TokenKind::Identifier(identifier) => write!(f, "\"{identifier}\""),
			TokenKind::Integer(integer) => write!(f, "{integer}"),
			TokenKind::Assign => write!(f, "="),
			TokenKind::Plus => write!(f, "+"),
			TokenKind::Minus => write!(f, "-"),
			TokenKind::Slash => write!(f, "/"),
			TokenKind::Asterisk => write!(f, "*"),
			TokenKind::Modulo => write!(f, "%"),
			TokenKind::Bang => write!(f, "!"),
			TokenKind::Equal => write!(f, "=="),
			TokenKind::NotEqual => write!(f, "!="),
			TokenKind::LessThan => write!(f, "<"),
			TokenKind::GreatherThan => write!(f, ">"),
			TokenKind::LessThanOrEqual => write!(f, "<="),
			TokenKind::GreatherThanOrEqual => write!(f, ">="),
			TokenKind::Comma => write!(f, ","),
			TokenKind::Semicolon => write!(f, ";"),
			TokenKind::LeftParen => write!(f, "("),
			TokenKind::RightParen => write!(f, ")"),
			TokenKind::LeftBrace => write!(f, "{{"),
			TokenKind::RightBrace => write!(f, "}}"),
			TokenKind::Let => write!(f, "let"),
			TokenKind::Function => write!(f, "fn"),
			TokenKind::Return => write!(f, "return"),
			TokenKind::If => write!(f, "if"),
			TokenKind::Else => write!(f, "else"),
			TokenKind::True => write!(f, "true"),
			TokenKind::False => write!(f, "false"),
		}
	}
}

impl From<char> for TokenKind {
	fn from(value: char) -> Self {
		Self::Illegal(value)
	}
}

impl From<&str> for TokenKind {
	fn from(value: &str) -> Self {
		Self::Identifier(value.to_owned())
	}
}

impl From<String> for TokenKind {
	fn from(value: String) -> Self {
		Self::Identifier(value)
	}
}

impl From<i64> for TokenKind {
	fn from(value: i64) -> Self {
		Self::Integer(value)
	}
}
