use crate::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
	Let { name: String, value: Expression },
	Return { value: Expression },
	Expression(Expression),
	Block(Vec<Statement>),
}

impl Statement {
	pub fn r#let(name: impl Into<String>, value: impl Into<Expression>) -> Self {
		Self::Let { name: name.into(), value: value.into() }
	}

	pub fn r#return(value: impl Into<Expression>) -> Self {
		Self::Return { value: value.into() }
	}

	pub fn expression(expression: impl Into<Expression>) -> Self {
		Self::Expression(expression.into())
	}
}

impl std::fmt::Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Statement::Let { name, value } => write!(f, "let {name} = {value};"),
			Statement::Return { value } => write!(f, "return {value};"),
			Statement::Expression(expression) => write!(f, "{expression}"),
			Statement::Block(statements) => {
				for statement in statements {
					write!(f, "{statement}")?;
				}

				Ok(())
			}
		}
	}
}
