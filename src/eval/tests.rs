use {
	crate::{
		ast::InfixOperator, eval::Environment, Eval, Expression, Parser, Statement, Tokenizer,
		Value,
	},
	color_eyre::{eyre::bail as yeet, Result},
	pretty_assertions::assert_eq,
	std::rc::Rc,
};

#[test]
fn eval_int_expression() -> Result<()> {
	let test_cases = [
		("5", 5),
		("10", 10),
		("-5", -5),
		("-10", -10),
		("5 + 5 + 5 + 5 - 10", 10),
		("2 * 2 * 2 * 2 * 2", 32),
		("-50 + 100 + -50", 0),
		("5 * 2 + 10", 20),
		("5 + 2 * 10", 25),
		("20 + 2 * -10", 0),
		("50 / 2 * 2 + 10", 60),
		("2 * (5 + 10)", 30),
		("3 * 3 * 3 + 10", 37),
		("3 * (3 * 3) + 10", 37),
		("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
	];

	for (input, value) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let environment = Rc::new(Environment::default());
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);

		let evaluated = program.eval(Rc::clone(&environment))?;
		assert_eq!(evaluated, Value::Int(value));
	}

	Ok(())
}

#[test]
fn eval_bool_expression() -> Result<()> {
	let test_cases = [
		("true", true),
		("false", false),
		("!true", false),
		("!false", true),
		("!5", false),
		("!!true", true),
		("!!false", false),
		("!!5", true),
		("1 < 2", true),
		("1 > 2", false),
		("1 < 1", false),
		("1 > 1", false),
		("1 == 1", true),
		("1 != 1", false),
		("1 == 2", false),
		("1 != 2", true),
		("true == true", true),
		("false == false", true),
		("true == false", false),
		("true != false", true),
		("false != true", true),
		("(1 < 2) == true", true),
		("(1 < 2) == false", false),
		("(1 > 2) == true", false),
		("(1 > 2) == false", true),
	];

	for (input, value) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let environment = Rc::new(Environment::default());
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);

		let evaluated = program.eval(Rc::clone(&environment))?;
		assert_eq!(evaluated, Value::Bool(value));
	}

	Ok(())
}

#[test]
fn eval_if_else_expression() -> Result<()> {
	let test_cases = [
		("if (true) { 10 }", Value::Int(10)),
		("if (false) { 10 }", Value::Null),
		("if (1) { 10 }", Value::Int(10)),
		("if (1 < 2) { 10 }", Value::Int(10)),
		("if (1 > 2) { 10 }", Value::Null),
		("if (1 > 2) { 10 } else { 20 }", Value::Int(20)),
		("if (1 < 2) { 10 } else { 20 }", Value::Int(10)),
	];

	for (input, value) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let environment = Rc::new(Environment::default());
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);
		assert_eq!(program.statements.len(), 1);

		let evaluated = program.eval(Rc::clone(&environment))?;
		assert_eq!(evaluated, value);
	}

	Ok(())
}

#[test]
fn eval_return_expression() -> Result<()> {
	let test_cases = [
		("return 10;", 10),
		("return 10; 9;", 10),
		("return 2 * 5; 9;", 10),
		("9; return 2 * 5; 9;", 10),
		("if (10 > 1) { if (10 > 1) { return 10; }; return 1; }", 10),
	];

	for (input, value) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let environment = Rc::new(Environment::default());
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);

		let evaluated = program.eval(Rc::clone(&environment))?;
		assert_eq!(evaluated, Value::Int(value));
	}

	Ok(())
}

#[test]
fn let_bindings() -> Result<()> {
	let test_cases = [
		("let a = 5; a;", 5),
		("let a = 5 * 5; a;", 25),
		("let a = 5; let b = a; b;", 5),
		("let a = 5; let b = a; let c = a + b + 5; c;", 15),
	];

	for (input, value) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let environment = Rc::new(Environment::default());
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);

		let evaluated = program.eval(Rc::clone(&environment))?;
		assert_eq!(evaluated, Value::Int(value));
	}

	Ok(())
}

#[test]
fn function_values() -> Result<()> {
	let input = "fn(x) { x + 2; };";

	let tokenizer = Tokenizer::new(input.chars().collect());
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let environment = Rc::new(Environment::default());
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);

	let evaluated = program.eval(Rc::clone(&environment))?;
	let Value::Function { parameters, body, .. } = evaluated else {
		yeet!("Value was not a function but `{evaluated:?}`");
	};

	assert_eq!(parameters, vec![String::from("x")]);
	assert_eq!(body, vec![Statement::Expression(Expression::Infix {
		operator: InfixOperator::Add,
		lhs: Box::new("x".into()),
		rhs: Box::new(2.into())
	})]);

	Ok(())
}

#[test]
fn function_call_values() -> Result<()> {
	let test_cases = [
		("let identity = fn(x) { x; }; identity(5);", 5),
		("let identity = fn(x) { return x; }; identity(5);", 5),
		("let double = fn(x) { x * 2; }; double(5);", 10),
		("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
		("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
		("fn(x) { x; }(5)", 5),
	];

	for (input, expected) in test_cases {
		let tokenizer = Tokenizer::new(input.chars().collect());
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();
		let environment = Rc::new(Environment::default());
		let errors = parser.errors.len();

		assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);

		let evaluated = program.eval(Rc::clone(&environment))?;
		assert_eq!(evaluated, Value::Int(expected), "Failed on `{input}`");
	}

	Ok(())
}

#[test]
fn closures() -> Result<()> {
	let input = r#"
		let adder = fn(x) {
		  fn(y) { x + y };
		};

		let add_two = adder(2);
		add_two(2);
	"#;

	let tokenizer = Tokenizer::new(input.chars().collect());
	let mut parser = Parser::new(tokenizer)?;
	let program = parser.parse_program();
	let environment = Rc::new(Environment::default());
	let errors = parser.errors.len();

	assert_eq!(errors, 0, "Parser had {errors} errors: {:#?}", parser.errors);

	let evaluated = program.eval(Rc::clone(&environment))?;
	assert_eq!(evaluated, Value::Int(4));

	Ok(())
}
