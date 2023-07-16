use crate::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
	Lowest,
	Equals,
	LessGreater,
	Sum,
	Product,
	Prefix,
	Call,
}

impl From<&Token> for Precedence {
	#[tracing::instrument(level = "TRACE", ret)]
	fn from(token: &Token) -> Self {
		match token {
			Token::Equal | Token::NotEqual => Self::Equals,
			Token::Plus | Token::Minus => Self::Sum,
			Token::Asterisk | Token::Slash => Self::Product,
			Token::LessThan | Token::GreaterThan => Self::LessGreater,
			Token::LeftParen => Self::Call,
			_ => Self::Lowest,
		}
	}
}
