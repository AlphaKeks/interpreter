use {
	super::{InfixOperator, PrefixOperator},
	crate::Statement,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
	Int(i64),
	Bool(bool),
	Identifier(String),
	Condition {
		condition: Box<Expression>,
		consequence: Vec<Statement>,
		alternative: Option<Vec<Statement>>,
	},
	Function {
		parameters: Vec<String>,
		body: Vec<Statement>,
	},
	Call {
		function: Box<Expression>,
		arguments: Vec<Expression>,
	},
	Prefix {
		operator: PrefixOperator,
		rhs: Box<Expression>,
	},
	Infix {
		operator: InfixOperator,
		lhs: Box<Expression>,
		rhs: Box<Expression>,
	},
}

impl From<i64> for Expression {
	fn from(value: i64) -> Self {
		Self::Int(value)
	}
}

impl From<bool> for Expression {
	fn from(value: bool) -> Self {
		Self::Bool(value)
	}
}

impl From<&str> for Expression {
	fn from(value: &str) -> Self {
		Self::Identifier(value.to_owned())
	}
}

impl From<String> for Expression {
	fn from(value: String) -> Self {
		Self::Identifier(value.to_owned())
	}
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Expression::Int(int) => write!(f, "{int}"),
			Expression::Bool(bool) => write!(f, "{bool}"),
			Expression::Identifier(identifier) => write!(f, "{identifier}"),
			Expression::Condition { condition, consequence, alternative } => {
				write!(f, "if ({condition}) {{ ")?;

				for statement in consequence {
					write!(f, "{statement}")?;
				}

				write!(f, " }}")?;

				if let Some(alternative) = alternative {
					write!(f, " else {{ ")?;

					for statement in alternative {
						write!(f, "{statement}")?;
					}

					write!(f, " }}")?;
				}

				Ok(())
			}
			Expression::Function { parameters, body } => {
				write!(f, "fn ({}) {{ ", parameters.join(", "))?;

				for statement in body {
					write!(f, "{statement}")?;
				}

				write!(f, " }}")
			}
			Expression::Call { function, arguments } => {
				let arguments = arguments
					.iter()
					.map(|arg| arg.to_string())
					.collect::<Vec<_>>()
					.join(", ");

				write!(f, "{function}({arguments})")
			}
			Expression::Prefix { operator, rhs } => write!(f, "({operator}{rhs})"),
			Expression::Infix { operator, lhs, rhs } => write!(f, "({lhs} {operator} {rhs})"),
		}
	}
}
