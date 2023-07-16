use {
	crate::Token,
	color_eyre::{eyre::bail as yeet, Result},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOperator {
	Not,
	Neg,
}

impl std::fmt::Display for PrefixOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			PrefixOperator::Not => "!",
			PrefixOperator::Neg => "-",
		})
	}
}

impl TryFrom<&Token> for PrefixOperator {
	type Error = color_eyre::Report;

	#[tracing::instrument(level = "TRACE", ret)]
	fn try_from(token: &Token) -> Result<Self> {
		Ok(match token {
			Token::Bang => Self::Not,
			Token::Minus => Self::Neg,
			token => yeet!("`{token:?}` is not a valid prefix operator"),
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfixOperator {
	Add,
	Sub,
	Mul,
	Div,
	Modulo,
	Equal,
	NotEqual,
	GreaterThan,
	LessThan,
	GreaterThanOrEqual,
	LessThanOrEqual,
}

impl std::fmt::Display for InfixOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			InfixOperator::Add => "+",
			InfixOperator::Sub => "-",
			InfixOperator::Mul => "*",
			InfixOperator::Div => "/",
			InfixOperator::Modulo => "%",
			InfixOperator::Equal => "==",
			InfixOperator::NotEqual => "!=",
			InfixOperator::GreaterThan => ">",
			InfixOperator::LessThan => "<",
			InfixOperator::GreaterThanOrEqual => ">=",
			InfixOperator::LessThanOrEqual => "<=",
		})
	}
}
