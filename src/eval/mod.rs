#[cfg(test)]
mod tests;

use {
	crate::{
		ast::{InfixOperator, PrefixOperator},
		Expression, Program, Statement, Value,
	},
	color_eyre::{eyre::bail as yeet, Result},
};

mod environment;
use std::rc::Rc;

pub use environment::Environment;

pub trait Eval {
	fn eval(self, environment: Rc<Environment>) -> Result<Value>;
}

impl Eval for Program {
	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval(self, environment: Rc<Environment>) -> Result<Value> {
		let mut result = Value::Null;

		for statement in self.statements {
			result = statement.eval(Rc::clone(&environment))?;

			if let Value::Return(value) = result {
				return Ok(*value);
			}
		}

		Ok(result)
	}
}

impl Eval for Statement {
	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval(self, environment: Rc<Environment>) -> Result<Value> {
		match self {
			Statement::Expression(value) => value.eval(environment),

			Statement::Let { name, value } => {
				let value = value.eval(Rc::clone(&environment))?;
				let value = environment.set(name, value);
				Ok(value)
			}

			Statement::Return { value } => value
				.eval(environment)
				.map(|value| Value::Return(Box::new(value))),

			Statement::Block(statements) => {
				let mut result = Value::Null;

				for statement in statements {
					result = statement.eval(Rc::clone(&environment))?;

					if matches!(result, Value::Return(_)) {
						return Ok(result);
					}
				}

				Ok(result)
			}
		}
	}
}

impl Eval for Expression {
	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval(self, environment: Rc<Environment>) -> Result<Value> {
		Ok(match self {
			Expression::Int(int) => Value::Int(int),
			Expression::Bool(bool) => Value::Bool(bool),
			Expression::Identifier(identifier) if identifier == "null" => Value::Null,
			Expression::Identifier(identifier) => environment.get(identifier),
			Expression::Condition { condition, consequence, alternative } => {
				let condition = match condition.eval(Rc::clone(&environment))? {
					Value::Bool(bool) => bool,
					Value::Int(int) => int != 0,
					condition => yeet!(
						"Expected boolean expression inside conditional but got `{condition:?}`"
					),
				};

				if condition {
					Statement::Block(consequence).eval(environment)
				} else {
					alternative.map_or(Ok(Value::Null), |statements| {
						Statement::Block(statements).eval(environment)
					})
				}?
			}
			Expression::Function { parameters, body } => Value::Function {
				parameters,
				body,
				environment: Environment::with_outer(&environment),
			},
			Expression::Call { function, arguments } => {
				let evaluated = function.eval(environment)?;
				let Value::Function { parameters, body, environment: local_env } = evaluated else {
					yeet!("Expected function before call expression but got `{evaluated:?}`");
				};

				let local_env = Rc::new(local_env);
				let local_env = Rc::new(Environment::with_outer(&local_env));

				let arguments = arguments
					.into_iter()
					.map(|arg| dbg!(dbg!(arg).eval(Rc::clone(&local_env))))
					.collect::<Result<Vec<_>>>()?;

				let n_params = parameters.len();
				let n_args = arguments.len();
				if n_params < n_args {
					yeet!("Too many arguments! Expected {n_params} but got {n_args}");
				} else if n_params > n_args {
					yeet!("Not enough arguments! Expected {n_params} but got {n_args}");
				}

				for (param, arg) in parameters.into_iter().zip(arguments) {
					local_env.set(param, arg);
				}

				let body = match Statement::Block(body).eval(local_env)? {
					Value::Return(value) => *value,
					value => value,
				};

				body
			}
			Expression::Prefix { operator, rhs } => {
				let rhs = rhs.eval(environment)?;
				Expression::eval_prefix(operator, rhs)?
			}
			Expression::Infix { operator, lhs, rhs } => {
				let lhs = lhs.eval(Rc::clone(&environment))?;
				let rhs = rhs.eval(environment)?;
				Expression::eval_infix(operator, lhs, rhs)?
			}
		})
	}
}

impl Expression {
	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval_prefix(operator: PrefixOperator, rhs: Value) -> Result<Value> {
		Ok(match operator {
			PrefixOperator::Not => Self::eval_bang(rhs),
			PrefixOperator::Neg => Self::eval_neg(rhs)?,
		})
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval_bang(rhs: Value) -> Value {
		Value::Bool(match rhs {
			Value::Null => true,
			Value::Return(value) => return Self::eval_bang(*value),
			Value::Int(int) => int == 0,
			Value::Bool(bool) => !bool,
			Value::Function { .. } => false,
		})
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval_neg(rhs: Value) -> Result<Value> {
		Ok(Value::Int(match rhs {
			Value::Int(int) => -int,
			Value::Return(value) => return Self::eval_neg(*value),
			Value::Null | Value::Bool(_) | Value::Function { .. } => {
				yeet!("`{rhs:?}` cannot be negated");
			}
		}))
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn eval_infix(operator: InfixOperator, lhs: Value, rhs: Value) -> Result<Value> {
		Ok(match (lhs, rhs) {
			(Value::Null, Value::Null) => Value::Bool(match operator {
				InfixOperator::Equal => true,
				InfixOperator::NotEqual => false,
				operator => yeet!("Cannot perform operation `{operator:?}` on null"),
			}),
			(Value::Int(left), Value::Int(right)) => match operator {
				InfixOperator::Add => Value::Int(left + right),
				InfixOperator::Sub => Value::Int(left - right),
				InfixOperator::Mul => Value::Int(left * right),
				InfixOperator::Div => Value::Int(left / right),
				InfixOperator::Modulo => Value::Int(left % right),
				InfixOperator::Equal => Value::Bool(left == right),
				InfixOperator::NotEqual => Value::Bool(left != right),
				InfixOperator::GreaterThan => Value::Bool(left > right),
				InfixOperator::LessThan => Value::Bool(left < right),
				InfixOperator::GreaterThanOrEqual => Value::Bool(left >= right),
				InfixOperator::LessThanOrEqual => Value::Bool(left <= right),
			},
			(Value::Bool(left), Value::Bool(right)) => Value::Bool(match operator {
				InfixOperator::Equal => left == right,
				InfixOperator::NotEqual => left != right,
				operator => yeet!("Cannot perform operation `{operator:?}` on a boolean"),
			}),
			(Value::Int(int), Value::Bool(bool)) | (Value::Bool(bool), Value::Int(int)) => {
				let int = int != 0;
				Value::Bool(match operator {
					InfixOperator::Equal => int == bool,
					InfixOperator::NotEqual => int != bool,
					operator => {
						yeet!(
							"Cannot perform operation `{operator:?}` on a boolean and integer combo"
						);
					}
				})
			}
			(lhs, rhs) => yeet!("Cannot evaluate `{lhs:?} {operator} {rhs:?}`"),
		})
	}
}
