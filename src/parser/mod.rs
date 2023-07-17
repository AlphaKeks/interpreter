#[cfg(test)]
mod tests;

use {
	crate::{
		ast::{InfixOperator, PrefixOperator, Program},
		Expression, Statement, Token, Tokenizer,
	},
	color_eyre::{eyre::bail as yeet, Result},
	macros::assert_token,
	std::result::Result as StdResult,
};

mod macros;
pub mod precedence;
pub use precedence::Precedence;

pub struct Parser {
	tokenizer: Tokenizer,
	current_token: Token,
	peek_token: Token,
	pub errors: Vec<color_eyre::Report>,
}

impl Parser {
	#[tracing::instrument(level = "TRACE", ret)]
	pub fn new(mut tokenizer: Tokenizer) -> Result<Self> {
		let current_token = tokenizer.step()?;
		let peek_token = tokenizer.step()?;
		let errors = Vec::new();
		Ok(Self { tokenizer, current_token, peek_token, errors })
	}

	#[tracing::instrument(level = "TRACE", ret)]
	pub fn parse_program(&mut self) -> Program {
		let mut program = Program::default();

		while self.current_token != Token::Eof {
			match self.parse_statement() {
				Ok(statement) => program.statements.push(statement),
				Err(error) => self.errors.push(error),
			};

			if let Err(error) = self.step() {
				self.errors.push(error);
			}
		}

		program
	}
}

impl Parser {
	#[tracing::instrument(level = "DEBUG")]
	fn step(&mut self) -> Result<()> {
		self.current_token = std::mem::replace(&mut self.peek_token, self.tokenizer.step()?);
		Ok(())
	}

	#[tracing::instrument(level = "INFO", ret)]
	fn parse_statement(&mut self) -> Result<Statement> {
		match &self.current_token {
			Token::Let => self.parse_let(),
			Token::Return => self.parse_return(),
			_ => self.parse_expression_statement(),
		}
	}

	#[tracing::instrument(level = "INFO", ret)]
	fn parse_let(&mut self) -> Result<Statement> {
		let name = assert_token!(peek, self, Token::Ident(ident) => {
			ident.to_owned()
		});

		assert_token!(peek, self, Token::Assign);
		self.step()?;

		let value = self.parse_expression(Precedence::Lowest)?;

		if self.peek_token == Token::Semicolon {
			self.step()?;
		}

		Ok(Statement::Let { name, value })
	}

	#[tracing::instrument(level = "INFO", ret)]
	fn parse_return(&mut self) -> Result<Statement> {
		self.step()?;
		let value = self.parse_expression(Precedence::Lowest)?;
		let statement = Statement::Return { value };

		if self.peek_token == Token::Semicolon {
			self.step()?;
		}

		Ok(statement)
	}

	#[tracing::instrument(level = "INFO", ret)]
	fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
		let mut prefix = self.parse_prefix()?;

		while self.peek_token != Token::Semicolon && precedence < self.peek_token.precedence() {
			self.step()?;
			match self.parse_infix(prefix)? {
				Ok(expression) => prefix = expression,
				Err(expression) => return Ok(expression),
			};
		}

		Ok(prefix)
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_expression_statement(&mut self) -> Result<Statement> {
		let expression = self.parse_expression(Precedence::Lowest)?;

		if self.peek_token == Token::Semicolon {
			self.step()?;
		}

		Ok(Statement::Expression(expression))
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	pub fn parse_prefix(&mut self) -> Result<Expression> {
		let expression = match &self.current_token {
			Token::Minus | Token::Bang => self.parse_prefix_expression()?,
			Token::Int(int) => Expression::Int(*int),
			Token::Ident(identifier) => Expression::Identifier(identifier.to_owned()),
			Token::True => Expression::Bool(true),
			Token::False => Expression::Bool(false),
			Token::LeftParen => self.parse_grouped_expression()?,
			Token::If => self.parse_if_expression()?,
			Token::Function => self.parse_function()?,
			token => yeet!("We don't know how to parse `{token:?}`"),
		};

		Ok(expression)
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_prefix_expression(&mut self) -> Result<Expression> {
		let operator = PrefixOperator::try_from(&self.current_token)?;
		self.step()?;
		let rhs = self.parse_expression(Precedence::Prefix)?;

		Ok(Expression::Prefix { operator, rhs: Box::new(rhs) })
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_grouped_expression(&mut self) -> Result<Expression> {
		self.step()?;
		let expression = self.parse_expression(Precedence::Lowest)?;
		assert_token!(peek, self, Token::RightParen);
		Ok(expression)
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_if_expression(&mut self) -> Result<Expression> {
		assert_token!(peek, self, Token::LeftParen);
		self.step()?;

		let condition = self.parse_expression(Precedence::Lowest)?;

		assert_token!(peek, self, Token::RightParen);
		assert_token!(peek, self, Token::LeftBrace);

		let consequence = self.parse_block()?;
		let mut alternative = None;

		if self.peek_token == Token::Else {
			self.step()?;
			assert_token!(peek, self, Token::LeftBrace);
			let block = self.parse_block()?;
			alternative = Some(block);
		}

		Ok(Expression::Condition { condition: Box::new(condition), consequence, alternative })
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_function(&mut self) -> Result<Expression> {
		assert_token!(peek, self, Token::LeftParen);

		let parameters = self.parse_function_parameters()?;

		assert_token!(peek, self, Token::LeftBrace);

		let body = self.parse_block()?;

		Ok(Expression::Function { parameters, body })
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_function_parameters(&mut self) -> Result<Vec<String>> {
		let mut identifiers = Vec::new();

		self.step()?;
		if self.current_token == Token::RightParen {
			return Ok(identifiers);
		}

		assert_token!(current, self, Token::Ident(identifier) => {
			identifiers.push(identifier.to_owned());
		});

		while self.peek_token == Token::Comma {
			self.step()?;
			self.step()?;
			assert_token!(current, self, Token::Ident(identifier) => {
				identifiers.push(identifier.to_owned());
			});
		}

		dbg!(&self);
		assert_token!(peek, self, Token::RightParen);

		Ok(identifiers)
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_block(&mut self) -> Result<Vec<Statement>> {
		let mut statements = Vec::new();
		self.step()?;

		while !matches!(self.current_token, Token::RightBrace | Token::Eof) {
			let statement = self.parse_statement()?;
			statements.push(statement);
			self.step()?;
		}

		Ok(statements)
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_call(&mut self, function: Expression) -> Result<Expression> {
		if !matches!(function, Expression::Identifier(_) | Expression::Function { .. }) {
			yeet!("Invalid function expression `{function:?}`");
		}

		let arguments = self.parse_call_arguments()?;
		Ok(Expression::Call { function: Box::new(function), arguments })
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	fn parse_call_arguments(&mut self) -> Result<Vec<Expression>> {
		let mut arguments = Vec::new();

		self.step()?;
		if self.current_token == Token::RightParen {
			return Ok(arguments);
		}

		arguments.push(self.parse_expression(Precedence::Lowest)?);
		while self.peek_token == Token::Comma {
			self.step()?;
			self.step()?;
			arguments.push(self.parse_expression(Precedence::Lowest)?);
		}

		assert_token!(peek, self, Token::RightParen);

		Ok(arguments)
	}

	#[tracing::instrument(level = "DEBUG", ret)]
	pub fn parse_infix(&mut self, lhs: Expression) -> Result<StdResult<Expression, Expression>> {
		let operator = match &self.current_token {
			Token::Equal => InfixOperator::Equal,
			Token::NotEqual => InfixOperator::NotEqual,
			Token::Plus => InfixOperator::Add,
			Token::Minus => InfixOperator::Sub,
			Token::Asterisk => InfixOperator::Mul,
			Token::Slash => InfixOperator::Div,
			Token::LessThan if self.peek_token == Token::Assign => InfixOperator::LessThanOrEqual,
			Token::LessThan => InfixOperator::LessThan,
			Token::GreaterThan if self.peek_token == Token::Assign => {
				InfixOperator::GreaterThanOrEqual
			}
			Token::GreaterThan => InfixOperator::GreaterThan,
			Token::LeftParen => return Ok(Ok(self.parse_call(lhs)?)),
			_ => return Ok(Err(lhs)),
		};

		let precedence = self.current_token.precedence();
		self.step()?;
		let rhs = self.parse_expression(precedence)?;
		Ok(Ok(Expression::Infix { operator, lhs: Box::new(lhs), rhs: Box::new(rhs) }))
	}
}

impl std::fmt::Debug for Parser {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{{ current: {:?} ({:?}), peek: {:?} ({:?}) }}",
			self.current_token,
			self.current_token.precedence(),
			self.peek_token,
			self.peek_token.precedence(),
		)
	}
}
