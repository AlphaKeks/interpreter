use {
	crate::{
		ast::{InfixOperator, PrefixOperator},
		Expression, Parser, Statement, Tokenizer,
	},
	color_eyre::{eyre::bail as yeet, Result},
	pretty_assertions::assert_eq,
};

#[test]
fn let_statements() -> Result<()> {
	let input = r#"
		let x = 5;
		let y = 10;
		let foobar = 838383;
	"#
	.chars()
	.collect();

	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 3);
	assert_eq!(program.statements[0], Statement::r#let("x", 5));
	assert_eq!(program.statements[1], Statement::r#let("y", 10));
	assert_eq!(program.statements[2], Statement::r#let("foobar", 838383));

	Ok(())
}

#[test]
fn return_statements() -> Result<()> {
	let input = r#"
		return 5;
		return 10;
		return 993322;
	"#
	.chars()
	.collect();

	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 3);
	assert_eq!(program.statements[0], Statement::r#return(5));
	assert_eq!(program.statements[1], Statement::r#return(10));
	assert_eq!(program.statements[2], Statement::r#return(993322));

	Ok(())
}

#[test]
fn identifier_expression() -> Result<()> {
	let input = "foobar;".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	assert_eq!(program.statements[0], Statement::expression("foobar"));

	Ok(())
}

#[test]
fn integer_literal_expression() -> Result<()> {
	let input = "5;".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	assert_eq!(program.statements[0], Statement::expression(5));

	Ok(())
}

#[test]
fn boolean_literal_expression() -> Result<()> {
	let test_cases = [("true;", Expression::Bool(true)), ("false;", Expression::Bool(false))];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);
		assert_eq!(program.statements[0], Statement::Expression(expected));
	}

	Ok(())
}

#[test]
fn string_expression() -> Result<()> {
	let input = "\"foobar\";".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	assert_eq!(
		program.statements[0],
		Statement::Expression(Expression::String(String::from("foobar")))
	);

	Ok(())
}

#[test]
fn array_expression() -> Result<()> {
	let input = "[1, 2 * 2, 3 + 3]".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	assert_eq!(
		program.statements[0],
		Statement::Expression(Expression::Array(vec![
			Expression::Int(1),
			Expression::Infix {
				operator: InfixOperator::Mul,
				lhs: Box::new(2.into()),
				rhs: Box::new(2.into()),
			},
			Expression::Infix {
				operator: InfixOperator::Add,
				lhs: Box::new(3.into()),
				rhs: Box::new(3.into()),
			}
		]))
	);

	Ok(())
}

#[test]
fn map_expression() -> Result<()> {
	let test_cases = [
		("{}", Expression::Map(Vec::new())),
		(
			r#"
				{
					"one": 1,
					"two": 2,
					"three": 3
				}
			"#,
			Expression::Map(vec![
				(Expression::String("one".into()), 1.into()),
				(Expression::String("two".into()), 2.into()),
				(Expression::String("three".into()), 3.into()),
			]),
		),
		(
			r#"
				{
					"one": 0 + 1,
					"two": 10 - 8,
					"three": 15 / 5
				}
			"#,
			Expression::Map(vec![
				(Expression::String("one".into()), Expression::Infix {
					operator: InfixOperator::Add,
					lhs: Box::new(0.into()),
					rhs: Box::new(1.into()),
				}),
				(Expression::String("two".into()), Expression::Infix {
					operator: InfixOperator::Sub,
					lhs: Box::new(10.into()),
					rhs: Box::new(8.into()),
				}),
				(Expression::String("three".into()), Expression::Infix {
					operator: InfixOperator::Div,
					lhs: Box::new(15.into()),
					rhs: Box::new(5.into()),
				}),
			]),
		),
	];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let mut program = parser.parse_program();
		let errors = parser.errors.len();

		eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);
		assert_eq!(program.statements.remove(0), Statement::Expression(expected));
	}

	Ok(())
}

#[test]
fn parsing_prefix_expressions() -> Result<()> {
	let test_cases = [
		("!5;", Expression::Prefix {
			operator: PrefixOperator::Not,
			rhs: Box::new(Expression::Int(5)),
		}),
		("-15;", Expression::Prefix {
			operator: PrefixOperator::Neg,
			rhs: Box::new(Expression::Int(15)),
		}),
		("!true;", Expression::Prefix {
			operator: PrefixOperator::Not,
			rhs: Box::new(Expression::Bool(true)),
		}),
		("!false;", Expression::Prefix {
			operator: PrefixOperator::Not,
			rhs: Box::new(Expression::Bool(false)),
		}),
	];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let mut program = parser.parse_program();
		let errors = parser.errors.len();

		eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);
		let statement = program.statements.remove(0);
		let Statement::Expression(expression) = statement else {
			yeet!("Statement was not an expression ({statement:?})");
		};

		assert_eq!(expression, expected);
	}

	Ok(())
}

#[test]
fn parsing_infix_expressions() -> Result<()> {
	let test_cases = [
		("5 + 5;", Expression::Infix {
			operator: InfixOperator::Add,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 - 5;", Expression::Infix {
			operator: InfixOperator::Sub,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 * 5;", Expression::Infix {
			operator: InfixOperator::Mul,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 / 5;", Expression::Infix {
			operator: InfixOperator::Div,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 > 5;", Expression::Infix {
			operator: InfixOperator::GreaterThan,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 < 5;", Expression::Infix {
			operator: InfixOperator::LessThan,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 == 5;", Expression::Infix {
			operator: InfixOperator::Equal,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("5 != 5;", Expression::Infix {
			operator: InfixOperator::NotEqual,
			lhs: Box::new(5.into()),
			rhs: Box::new(5.into()),
		}),
		("true == true", Expression::Infix {
			operator: InfixOperator::Equal,
			lhs: Box::new(true.into()),
			rhs: Box::new(true.into()),
		}),
		("true != false", Expression::Infix {
			operator: InfixOperator::NotEqual,
			lhs: Box::new(true.into()),
			rhs: Box::new(false.into()),
		}),
		("false == false", Expression::Infix {
			operator: InfixOperator::Equal,
			lhs: Box::new(false.into()),
			rhs: Box::new(false.into()),
		}),
	];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let mut program = parser.parse_program();
		let errors = parser.errors.len();

		eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);
		let statement = program.statements.remove(0);
		let Statement::Expression(expression) = statement else {
			yeet!("Statement was not an expression ({statement:?})");
		};

		assert_eq!(expression, expected);
	}

	Ok(())
}

#[test]
fn operator_precedence() -> Result<()> {
	let test_cases = [
		("-a * b", "((-a) * b)"),
		("!-a", "(!(-a))"),
		("a + b + c", "((a + b) + c)"),
		("a + b - c", "((a + b) - c)"),
		("a * b * c", "((a * b) * c)"),
		("a * b / c", "((a * b) / c)"),
		("a + b / c", "(a + (b / c))"),
		("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
		("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
		("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
		("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
		("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
		("true", "true"),
		("false", "false"),
		("3 > 5 == false", "((3 > 5) == false)"),
		("3 < 5 == true", "((3 < 5) == true)"),
		("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
		("(5 + 5) * 2", "((5 + 5) * 2)"),
		("2 / (5 + 5)", "(2 / (5 + 5))"),
		("-(5 + 5)", "(-(5 + 5))"),
		("!(true == true)", "(!(true == true))"),
		("a + add(b * c) + d", "((a + add((b * c))) + d)"),
		(
			"add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
			"add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
		),
		("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))"),
		("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)"),
		("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
	];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let errors = parser.errors.len();

		eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
		assert_eq!(program.to_string(), expected);
	}

	Ok(())
}

#[test]
fn parsing_if_expressions() -> Result<()> {
	let input = "if (x < y) { x }".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let mut program = parser.parse_program();
	let errors = parser.errors.len();

	eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	let statement = program.statements.remove(0);
	let Statement::Expression(expression) = statement else {
		yeet!("Statement was not an expression ({statement:?})");
	};

	assert_eq!(expression, Expression::Condition {
		condition: Box::new(Expression::Infix {
			operator: InfixOperator::LessThan,
			lhs: Box::new("x".into()),
			rhs: Box::new("y".into())
		}),
		consequence: vec![Statement::expression("x")],
		alternative: None,
	});

	Ok(())
}

#[test]
fn parsing_if_else_expressions() -> Result<()> {
	let input = "if (x < y) { x } else { y }".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let mut program = parser.parse_program();
	let errors = parser.errors.len();

	eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	let statement = program.statements.remove(0);
	let Statement::Expression(expression) = statement else {
		yeet!("Statement was not an expression ({statement:?})");
	};

	assert_eq!(expression, Expression::Condition {
		condition: Box::new(Expression::Infix {
			operator: InfixOperator::LessThan,
			lhs: Box::new("x".into()),
			rhs: Box::new("y".into())
		}),
		consequence: vec![Statement::expression("x")],
		alternative: Some(vec![Statement::expression("y")]),
	});

	Ok(())
}

#[test]
fn parsing_function_expressions() -> Result<()> {
	let input = "fn(x, y) { x + y; }".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let mut program = parser.parse_program();
	let errors = parser.errors.len();

	eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	let statement = program.statements.remove(0);
	let Statement::Expression(expression) = statement else {
		yeet!("Statement was not an expression ({statement:?})");
	};

	assert_eq!(expression, Expression::Function {
		parameters: vec![String::from("x"), String::from("y")],
		body: vec![Statement::Expression(Expression::Infix {
			operator: InfixOperator::Add,
			lhs: Box::new("x".into()),
			rhs: Box::new("y".into())
		})],
	});

	Ok(())
}

#[test]
fn parsing_function_parameters() -> Result<()> {
	let test_cases = [
		("fn() {};", vec![]),
		("fn(x) {};", vec![String::from("x")]),
		("fn(x, y, z) {};", vec![String::from("x"), String::from("y"), String::from("z")]),
	];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let mut program = parser.parse_program();
		let errors = parser.errors.len();

		eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);
		let statement = program.statements.remove(0);
		let Statement::Expression(expression) = statement else {
			yeet!("Statement was not an expression ({statement:?})");
		};

		assert_eq!(expression, Expression::Function { parameters: expected, body: vec![] });
	}

	Ok(())
}

#[test]
fn parsing_call_expressions() -> Result<()> {
	let input = "add(1, 2 * 3, 4 + 5);".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let mut program = parser.parse_program();
	let errors = parser.errors.len();

	eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	let statement = program.statements.remove(0);
	let Statement::Expression(expression) = statement else {
		yeet!("Statement was not an expression ({statement:?})");
	};

	assert_eq!(expression, Expression::Call {
		function: Box::new("add".into()),
		arguments: vec![
			1.into(),
			Expression::Infix {
				operator: InfixOperator::Mul,
				lhs: Box::new(2.into()),
				rhs: Box::new(3.into()),
			},
			Expression::Infix {
				operator: InfixOperator::Add,
				lhs: Box::new(4.into()),
				rhs: Box::new(5.into()),
			},
		],
	});

	Ok(())
}

#[test]
fn parsing_index_expressions() -> Result<()> {
	let input = "my_array[1 + 1]".chars().collect();
	let tokenizer = Tokenizer::new(input);
	let mut parser = Parser::new(tokenizer)?;
	let mut program = parser.parse_program();
	let errors = parser.errors.len();

	eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
	assert_eq!(program.statements.len(), 1);
	let statement = program.statements.remove(0);
	let Statement::Expression(expression) = statement else {
		yeet!("Statement was not an expression ({statement:?})");
	};

	assert_eq!(expression, Expression::Index {
		lhs: Box::new("my_array".into()),
		idx: Box::new(Expression::Infix {
			operator: InfixOperator::Add,
			lhs: Box::new(1.into()),
			rhs: Box::new(1.into()),
		})
	});

	Ok(())
}

#[test]
fn parsing_call_parameters() -> Result<()> {
	let test_cases = [
		("add();", "add", vec![]),
		("add(1);", "add", vec![Expression::Int(1)]),
		("add(1, 2 * 3, 4 + 5);", "add", vec![
			Expression::Int(1),
			Expression::Infix {
				operator: InfixOperator::Mul,
				lhs: Box::new(2.into()),
				rhs: Box::new(3.into()),
			},
			Expression::Infix {
				operator: InfixOperator::Add,
				lhs: Box::new(4.into()),
				rhs: Box::new(5.into()),
			},
		]),
	];

	for (input, identifier, arguments) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let mut program = parser.parse_program();
		let errors = parser.errors.len();

		eprintln!("Parser had {errors} error(s): {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);
		let statement = program.statements.remove(0);
		let Statement::Expression(expression) = statement else {
			yeet!("Statement was not an expression ({statement:?})");
		};

		assert_eq!(expression, Expression::Call {
			function: Box::new(identifier.into()),
			arguments
		});
	}

	Ok(())
}
