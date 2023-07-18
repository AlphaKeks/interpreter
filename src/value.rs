use crate::{eval::Environment, Statement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
	Null,
	Return(Box<Value>),
	Int(i64),
	Bool(bool),
	Function { parameters: Vec<String>, body: Vec<Statement>, environment: Environment },
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Null => write!(f, "null"),
			Value::Return(value) => write!(f, "{value}"),
			Value::Int(int) => write!(f, "{int}"),
			Value::Bool(bool) => write!(f, "{bool}"),
			Value::Function { parameters, body, .. } => {
				write!(
					f,
					"fn({}) {{\n  {}\n}}",
					parameters.join(", "),
					body.iter()
						.map(|statement| statement.to_string())
						.collect::<Vec<_>>()
						.join("\n  ")
				)
			}
		}
	}
}
