use crate::parser::Precedence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	/// `=`
	Assign,

	/// `==`
	Equal,

	/// `!=`
	NotEqual,

	/// `+`
	Plus,

	/// `-`
	Minus,

	/// `!`
	Bang,

	/// `*`
	Asterisk,

	/// `/`
	Slash,

	/// `<`
	LessThan,

	/// `>`
	GreaterThan,

	/// `,`
	Comma,

	/// `:`
	Colon,

	/// `;`
	Semicolon,

	/// `(`
	LeftParen,

	/// `)`
	RightParen,

	/// `{`
	LeftBrace,

	/// `}`
	RightBrace,

	/// `[`
	LeftBracket,

	/// `]`
	RightBracket,

	/// The end of the input.
	Eof,

	/// Any integer.
	Int(i64),

	/// Any identifier.
	Ident(String),

	/// String literals
	String(String),

	/// The `fn` keyword.
	Function,

	/// The `let` keyword.
	Let,

	/// The `if` keyword.
	If,

	/// The `else` keyword.
	Else,

	/// The `return` keyword.
	Return,

	/// The `true` keyword.
	True,

	/// The `false` keyword.
	False,

	/// Any unexpected token.
	Illegal(String),
}

impl Token {
	pub fn int(int: impl Into<i64>) -> Self {
		Self::Int(int.into())
	}

	pub fn ident(ident: impl Into<String>) -> Self {
		Self::Ident(ident.into())
	}

	pub fn illegal(illegal: impl Into<String>) -> Self {
		Self::Illegal(illegal.into())
	}

	pub fn string(string: impl Into<String>) -> Self {
		Self::String(string.into())
	}

	pub fn precedence(&self) -> Precedence {
		self.into()
	}
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Assign => write!(f, "="),
			Token::Equal => write!(f, "=="),
			Token::NotEqual => write!(f, "!="),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Bang => write!(f, "!"),
			Token::Asterisk => write!(f, "*"),
			Token::Slash => write!(f, "/"),
			Token::LessThan => write!(f, "<"),
			Token::GreaterThan => write!(f, ">"),
			Token::Comma => write!(f, ","),
			Token::Colon => write!(f, ":"),
			Token::Semicolon => write!(f, ";"),
			Token::LeftParen => write!(f, "("),
			Token::RightParen => write!(f, ")"),
			Token::LeftBrace => write!(f, "{{"),
			Token::RightBrace => write!(f, "}}"),
			Token::LeftBracket => write!(f, "["),
			Token::RightBracket => write!(f, "]"),
			Token::Int(int) => write!(f, "{int}"),
			Token::Ident(ident) => write!(f, "{ident}"),
			Token::String(string) => write!(f, "\"{string}\""),
			Token::Function => write!(f, "fn"),
			Token::Let => write!(f, "let"),
			Token::If => write!(f, "if"),
			Token::Else => write!(f, "else"),
			Token::Return => write!(f, "return"),
			Token::True => write!(f, "true"),
			Token::False => write!(f, "false"),
			Token::Eof => write!(f, "EOF"),
			Token::Illegal(illegal) => write!(f, "{illegal}"),
		}
	}
}
