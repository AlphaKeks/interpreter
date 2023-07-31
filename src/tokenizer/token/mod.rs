use {phf::phf_map, std::fmt::Display};

mod kind;
pub use kind::TokenKind;

mod span;
pub use span::TokenSpan;

/// All built-in keywords.
pub static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
	"let" => TokenKind::Let,
	"fn" => TokenKind::Function,
	"return" => TokenKind::Return,
	"if" => TokenKind::If,
	"else" => TokenKind::Else,
	"true" => TokenKind::True,
	"false" => TokenKind::False,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
	pub kind: TokenKind,
	pub span: TokenSpan,
}

impl Token {
	pub fn new(kind: impl Into<TokenKind>, span: impl Into<TokenSpan>) -> Self {
		Self { kind: kind.into(), span: span.into() }
	}
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}:{}] {:?}", self.span.line, self.span.column, self.kind)
	}
}
