use {
	crate::{Token, Tokenizer},
	color_eyre::Result,
	pretty_assertions::assert_eq,
};

#[test]
fn simple() -> Result<()> {
	let input = "=+(){},;".chars().collect();
	let mut tokenizer = Tokenizer::new(input);
	let expected = [
		Token::Assign,
		Token::Plus,
		Token::LeftParen,
		Token::RightParen,
		Token::LeftBrace,
		Token::RightBrace,
		Token::Comma,
		Token::Semicolon,
		Token::Eof,
	];

	for expected in expected {
		assert_eq!(expected, tokenizer.step()?);
	}

	Ok(())
}

#[test]
fn basic_program() -> Result<()> {
	let input = r#"
		let five = 5;
		let ten = 10;

		let add = fn(x, y) {
			x + y;
		};

		let result = add(five, ten);
	"#
	.chars()
	.collect();

	let mut tokenizer = Tokenizer::new(input);
	let expected = [
		Token::Let,
		Token::ident("five"),
		Token::Assign,
		Token::Int(5),
		Token::Semicolon,
		Token::Let,
		Token::ident("ten"),
		Token::Assign,
		Token::Int(10),
		Token::Semicolon,
		Token::Let,
		Token::ident("add"),
		Token::Assign,
		Token::Function,
		Token::LeftParen,
		Token::ident("x"),
		Token::Comma,
		Token::ident("y"),
		Token::RightParen,
		Token::LeftBrace,
		Token::ident("x"),
		Token::Plus,
		Token::ident("y"),
		Token::Semicolon,
		Token::RightBrace,
		Token::Semicolon,
		Token::Let,
		Token::ident("result"),
		Token::Assign,
		Token::ident("add"),
		Token::LeftParen,
		Token::ident("five"),
		Token::Comma,
		Token::ident("ten"),
		Token::RightParen,
		Token::Semicolon,
		Token::Eof,
	];

	for expected in expected {
		assert_eq!(expected, tokenizer.step()?);
	}

	Ok(())
}

#[test]
fn more_tokens() -> Result<()> {
	let input = r#"
		let five = 5;
		let ten = 10;

		let add = fn(x, y) {
			x + y;
		};

		let result = add(five, ten);
		!-/*5;
		5 < 10 > 5;

		if (5 < 10) {
			return true;
		} else {
			return false;
		}

		10 == 10;
		10 != 9;
		"foobar"
		"foo bar"
		[1, 2];
		{"foo": "bar"}
	"#
	.chars()
	.collect();

	let mut tokenizer = Tokenizer::new(input);
	let expected = [
		Token::Let,
		Token::ident("five"),
		Token::Assign,
		Token::Int(5),
		Token::Semicolon,
		Token::Let,
		Token::ident("ten"),
		Token::Assign,
		Token::Int(10),
		Token::Semicolon,
		Token::Let,
		Token::ident("add"),
		Token::Assign,
		Token::Function,
		Token::LeftParen,
		Token::ident("x"),
		Token::Comma,
		Token::ident("y"),
		Token::RightParen,
		Token::LeftBrace,
		Token::ident("x"),
		Token::Plus,
		Token::ident("y"),
		Token::Semicolon,
		Token::RightBrace,
		Token::Semicolon,
		Token::Let,
		Token::ident("result"),
		Token::Assign,
		Token::ident("add"),
		Token::LeftParen,
		Token::ident("five"),
		Token::Comma,
		Token::ident("ten"),
		Token::RightParen,
		Token::Semicolon,
		Token::Bang,
		Token::Minus,
		Token::Slash,
		Token::Asterisk,
		Token::Int(5),
		Token::Semicolon,
		Token::Int(5),
		Token::LessThan,
		Token::Int(10),
		Token::GreaterThan,
		Token::Int(5),
		Token::Semicolon,
		Token::If,
		Token::LeftParen,
		Token::Int(5),
		Token::LessThan,
		Token::Int(10),
		Token::RightParen,
		Token::LeftBrace,
		Token::Return,
		Token::True,
		Token::Semicolon,
		Token::RightBrace,
		Token::Else,
		Token::LeftBrace,
		Token::Return,
		Token::False,
		Token::Semicolon,
		Token::RightBrace,
		Token::Int(10),
		Token::Equal,
		Token::Int(10),
		Token::Semicolon,
		Token::Int(10),
		Token::NotEqual,
		Token::Int(9),
		Token::Semicolon,
		Token::string("foobar"),
		Token::string("foo bar"),
		Token::LeftBracket,
		Token::Int(1),
		Token::Comma,
		Token::Int(2),
		Token::RightBracket,
		Token::Semicolon,
		Token::LeftBrace,
		Token::string("foo"),
		Token::Colon,
		Token::string("bar"),
		Token::RightBrace,
		Token::Eof,
	];

	for expected in expected {
		assert_eq!(expected, tokenizer.step()?);
	}

	Ok(())
}
