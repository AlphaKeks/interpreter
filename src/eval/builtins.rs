use {
	crate::Value,
	lazy_static::lazy_static,
	std::{collections::HashSet, fmt::Debug},
};

lazy_static! {
	pub static ref BUILTINS: HashSet<&'static str> =
		HashSet::from_iter(["print", "measure", "first"]);
}

pub trait BuiltinFunction: Debug {
	fn name(&self) -> String;
	fn call(&self, arguments: Vec<Value>) -> Value;
}

#[derive(Debug)]
pub struct Print;

impl BuiltinFunction for Print {
	fn name(&self) -> String {
		String::from("print")
	}

	fn call(&self, arguments: Vec<Value>) -> Value {
		let result = arguments
			.into_iter()
			.map(|arg| arg.to_string())
			.collect::<Vec<_>>()
			.join(", ");

		println!("{result}");

		Value::Null
	}
}

#[derive(Debug)]
pub struct Measure;

impl BuiltinFunction for Measure {
	fn name(&self) -> String {
		String::from("measure")
	}

	fn call(&self, arguments: Vec<Value>) -> Value {
		let is_empty = arguments.is_empty();
		let is_invalid = arguments
			.iter()
			.any(|value| !matches!(value, Value::String(_) | Value::Array(_)));

		if is_empty || is_invalid {
			return Value::Null;
		}

		let mut lens = arguments
			.into_iter()
			.map(|value| match value {
				Value::String(string) => Value::Int(string.len() as i64),
				Value::Array(array) => Value::Int(array.len() as i64),
				value => panic!("Expected string or array, but got `{value:?}`"),
			})
			.collect::<Vec<_>>();

		if lens.len() > 1 { Value::Array(lens) } else { lens.remove(0) }
	}
}

#[derive(Debug)]
pub struct First;

impl BuiltinFunction for First {
	fn name(&self) -> String {
		String::from("first")
	}

	fn call(&self, mut arguments: Vec<Value>) -> Value {
		match arguments.len() {
			0 => Value::Null,
			1 => {
				let Value::Array(mut array) = arguments.remove(0) else {
					return Value::Null;
				};

				if array.is_empty() {
					return Value::Null;
				}

				array.remove(0)
			}
			_ => {
				if arguments
					.iter()
					.any(|value| !matches!(value, Value::Array(_)))
				{
					return Value::Null;
				}

				let values = arguments
					.into_iter()
					.map(|array| {
						let Value::Array(mut array) = array else {
							panic!("Expected array");
						};

						if array.is_empty() { Value::Null } else { array.remove(0) }
					})
					.collect::<Vec<_>>();

				Value::Array(values)
			}
		}
	}
}
