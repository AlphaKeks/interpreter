use {
	crate::{
		eval::{builtins::BuiltinFunction, Environment},
		Statement,
	},
	std::{collections::HashMap, rc::Rc},
};

#[derive(Debug, Clone)]
pub enum Value {
	Null,
	Return(Box<Value>),
	Int(i64),
	Bool(bool),
	String(String),
	Array(Vec<Value>),
	Map(HashMap<String, Value>),
	Function { parameters: Vec<String>, body: Vec<Statement>, environment: Environment },
	BuiltinFunction(Rc<dyn BuiltinFunction>),
}

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Value::Null, Value::Null) => true,
			(Value::Return(left), Value::Return(right)) => left == right,
			(Value::Int(left), Value::Int(right)) => left == right,
			(Value::Bool(left), Value::Bool(right)) => left == right,
			(Value::String(left), Value::String(right)) => left == right,
			(Value::Array(left), Value::Array(right)) => left == right,
			(Value::Map(left), Value::Map(right)) => left == right,
			(
				Value::Function { environment: environment1, .. },
				Value::Function { environment: environment2, .. },
			) => environment1 == environment2,
			_ => false,
		}
	}
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Null => write!(f, "null"),
			Value::Return(value) => write!(f, "{value}"),
			Value::Int(int) => write!(f, "{int}"),
			Value::Bool(bool) => write!(f, "{bool}"),
			Value::String(string) => write!(f, "{string}"),
			Value::Array(values) => {
				write!(
					f,
					"[{}]",
					values
						.iter()
						.map(|value| value.to_string())
						.collect::<Vec<_>>()
						.join(", ")
				)
			}
			Value::Map(map) => {
				write!(f, "{{\n")?;

				for (k, v) in map {
					write!(f, "  {k} => {v},\n")?;
				}

				write!(f, "}}")
			}
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
			Value::BuiltinFunction(function) => {
				write!(f, "{}", function.name())
			}
		}
	}
}
