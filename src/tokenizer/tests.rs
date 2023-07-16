use {
	crate::{Token::*, Tokenizer},
	color_eyre::Result,
	pretty_assertions::assert_eq,
};

#[ctor::ctor]
fn setup() {
	color_eyre::install().expect("Failed to setup color-eyre");
	crate::tracing::init();
}

#[test]
fn simple() -> Result<()> {
	let input = "=+(){},;".chars().collect();
	let mut tokenizer = Tokenizer::new(input);
	let expected =
		[Assign, Plus, LeftParen, RightParen, LeftBrace, RightBrace, Comma, Semicolon, Eof];

	for expected in expected {
		assert_eq!(expected, tokenizer.next_token()?);
	}

	Ok(())
}

#[test]
fn basic_program() -> Result<()> {
	let input = include_str!("../../monkey/1_2.monkey")
		.chars()
		.collect();

	let mut tokenizer = Tokenizer::new(input);
	let expected = [
		Let,
		Ident(String::from("five")),
		Assign,
		Int(5),
		Semicolon,
		Let,
		Ident(String::from("ten")),
		Assign,
		Int(10),
		Semicolon,
		Let,
		Ident(String::from("add")),
		Assign,
		Function,
		LeftParen,
		Ident(String::from("x")),
		Comma,
		Ident(String::from("y")),
		RightParen,
		LeftBrace,
		Ident(String::from("x")),
		Plus,
		Ident(String::from("y")),
		Semicolon,
		RightBrace,
		Semicolon,
		Let,
		Ident(String::from("result")),
		Assign,
		Ident(String::from("add")),
		LeftParen,
		Ident(String::from("five")),
		Comma,
		Ident(String::from("ten")),
		RightParen,
		Semicolon,
		Eof,
	];

	for expected in expected {
		assert_eq!(expected, tokenizer.next_token()?);
	}

	Ok(())
}

#[test]
fn more_tokens() -> Result<()> {
	let input = include_str!("../../monkey/1_4.monkey")
		.chars()
		.collect();

	let mut tokenizer = Tokenizer::new(input);
	let expected = [
		Let,
		Ident(String::from("five")),
		Assign,
		Int(5),
		Semicolon,
		Let,
		Ident(String::from("ten")),
		Assign,
		Int(10),
		Semicolon,
		Let,
		Ident(String::from("add")),
		Assign,
		Function,
		LeftParen,
		Ident(String::from("x")),
		Comma,
		Ident(String::from("y")),
		RightParen,
		LeftBrace,
		Ident(String::from("x")),
		Plus,
		Ident(String::from("y")),
		Semicolon,
		RightBrace,
		Semicolon,
		Let,
		Ident(String::from("result")),
		Assign,
		Ident(String::from("add")),
		LeftParen,
		Ident(String::from("five")),
		Comma,
		Ident(String::from("ten")),
		RightParen,
		Semicolon,
		Bang,
		Minus,
		Slash,
		Asterisk,
		Int(5),
		Semicolon,
		Int(5),
		LessThan,
		Int(10),
		GreaterThan,
		Int(5),
		Semicolon,
		If,
		LeftParen,
		Int(5),
		LessThan,
		Int(10),
		RightParen,
		LeftBrace,
		Return,
		True,
		Semicolon,
		RightBrace,
		Else,
		LeftBrace,
		Return,
		False,
		Semicolon,
		RightBrace,
		Int(10),
		Equal,
		Int(10),
		Semicolon,
		Int(10),
		NotEqual,
		Int(9),
		Semicolon,
		Eof,
	];

	for expected in expected {
		assert_eq!(expected, tokenizer.next_token()?);
	}

	Ok(())
}
