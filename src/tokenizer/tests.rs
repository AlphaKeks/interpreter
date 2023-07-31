use {
	super::{token::TokenKind, Token, Tokenizer},
	color_eyre::Result,
	pretty_assertions::assert_eq,
};

#[test]
fn special_characters() -> Result<()> {
	let input = r"
		=+(){},;
	";

	let expected = [
		Token::new(TokenKind::Assign, (2, 3)),
		Token::new(TokenKind::Plus, (2, 4)),
		Token::new(TokenKind::LeftParen, (2, 5)),
		Token::new(TokenKind::RightParen, (2, 6)),
		Token::new(TokenKind::LeftBrace, (2, 7)),
		Token::new(TokenKind::RightBrace, (2, 8)),
		Token::new(TokenKind::Comma, (2, 9)),
		Token::new(TokenKind::Semicolon, (2, 10)),
	];

	let tokenizer = Tokenizer::new(input);
	let mut i = 0;

	for token in tokenizer {
		assert_eq!(token?, expected[i], "Failed at #{i}");
		i += 1;
	}

	assert_eq!(i, expected.len());

	Ok(())
}

#[test]
fn basic_program() -> Result<()> {
	let input = r"
		let five = 5;
		let ten = 10;

		let add = fn(x, y) {
			x + y
		};

		let result = add(five, ten);
	";

	let expected = [
		Token::new(TokenKind::Let, (2, 3)),
		Token::new("five", (2, 7)),
		Token::new(TokenKind::Assign, (2, 12)),
		Token::new(5, (2, 14)),
		Token::new(TokenKind::Semicolon, (2, 15)),
		Token::new(TokenKind::Let, (3, 3)),
		Token::new("ten", (3, 7)),
		Token::new(TokenKind::Assign, (3, 11)),
		Token::new(10, (3, 13)),
		Token::new(TokenKind::Semicolon, (3, 15)),
		Token::new(TokenKind::Let, (5, 3)),
		Token::new("add", (5, 7)),
		Token::new(TokenKind::Assign, (5, 11)),
		Token::new(TokenKind::Function, (5, 13)),
		Token::new(TokenKind::LeftParen, (5, 15)),
		Token::new("x", (5, 16)),
		Token::new(TokenKind::Comma, (5, 17)),
		Token::new("y", (5, 19)),
		Token::new(TokenKind::RightParen, (5, 20)),
		Token::new(TokenKind::LeftBrace, (5, 22)),
		Token::new("x", (6, 4)),
		Token::new(TokenKind::Plus, (6, 6)),
		Token::new("y", (6, 8)),
		Token::new(TokenKind::RightBrace, (7, 3)),
		Token::new(TokenKind::Semicolon, (7, 4)),
		Token::new(TokenKind::Let, (9, 3)),
		Token::new("result", (9, 7)),
		Token::new(TokenKind::Assign, (9, 14)),
		Token::new("add", (9, 16)),
		Token::new(TokenKind::LeftParen, (9, 19)),
		Token::new("five", (9, 20)),
		Token::new(TokenKind::Comma, (9, 24)),
		Token::new("ten", (9, 26)),
		Token::new(TokenKind::RightParen, (9, 29)),
		Token::new(TokenKind::Semicolon, (9, 30)),
	];

	let tokenizer = Tokenizer::new(input);
	let mut i = 0;

	for token in tokenizer {
		assert_eq!(token?, expected[i], "Failed at #{i}");
		i += 1;
	}

	assert_eq!(i, expected.len());

	Ok(())
}

#[test]
fn more_operators() -> Result<()> {
	let input = r"
		let five = 5;
		let ten = 10;

		let add = fn(x, y) {
			x + y
		};

		let result = add(five, ten);
		!-/*5;
		5 < 10 > 5;
	";

	let expected = [
		Token::new(TokenKind::Let, (2, 3)),
		Token::new("five", (2, 7)),
		Token::new(TokenKind::Assign, (2, 12)),
		Token::new(5, (2, 14)),
		Token::new(TokenKind::Semicolon, (2, 15)),
		Token::new(TokenKind::Let, (3, 3)),
		Token::new("ten", (3, 7)),
		Token::new(TokenKind::Assign, (3, 11)),
		Token::new(10, (3, 13)),
		Token::new(TokenKind::Semicolon, (3, 15)),
		Token::new(TokenKind::Let, (5, 3)),
		Token::new("add", (5, 7)),
		Token::new(TokenKind::Assign, (5, 11)),
		Token::new(TokenKind::Function, (5, 13)),
		Token::new(TokenKind::LeftParen, (5, 15)),
		Token::new("x", (5, 16)),
		Token::new(TokenKind::Comma, (5, 17)),
		Token::new("y", (5, 19)),
		Token::new(TokenKind::RightParen, (5, 20)),
		Token::new(TokenKind::LeftBrace, (5, 22)),
		Token::new("x", (6, 4)),
		Token::new(TokenKind::Plus, (6, 6)),
		Token::new("y", (6, 8)),
		Token::new(TokenKind::RightBrace, (7, 3)),
		Token::new(TokenKind::Semicolon, (7, 4)),
		Token::new(TokenKind::Let, (9, 3)),
		Token::new("result", (9, 7)),
		Token::new(TokenKind::Assign, (9, 14)),
		Token::new("add", (9, 16)),
		Token::new(TokenKind::LeftParen, (9, 19)),
		Token::new("five", (9, 20)),
		Token::new(TokenKind::Comma, (9, 24)),
		Token::new("ten", (9, 26)),
		Token::new(TokenKind::RightParen, (9, 29)),
		Token::new(TokenKind::Semicolon, (9, 30)),
		Token::new(TokenKind::Bang, (10, 3)),
		Token::new(TokenKind::Minus, (10, 4)),
		Token::new(TokenKind::Slash, (10, 5)),
		Token::new(TokenKind::Asterisk, (10, 6)),
		Token::new(5, (10, 7)),
		Token::new(TokenKind::Semicolon, (10, 8)),
		Token::new(5, (11, 3)),
		Token::new(TokenKind::LessThan, (11, 5)),
		Token::new(10, (11, 7)),
		Token::new(TokenKind::GreatherThan, (11, 10)),
		Token::new(5, (11, 12)),
		Token::new(TokenKind::Semicolon, (11, 13)),
	];

	let tokenizer = Tokenizer::new(input);
	let mut i = 0;

	for token in tokenizer {
		assert_eq!(token?, expected[i], "Failed at #{i}");
		i += 1;
	}

	assert_eq!(i, expected.len());

	Ok(())
}

#[test]
fn more_keywords() -> Result<()> {
	let input = r"
		let five = 5;
		let ten = 10;

		let add = fn(x, y) {
			x + y
		};

		let result = add(five, ten);
		!-/*5;
		5 < 10 > 5;

		if (5 < 10) {
			return true;
		} else {
			return false;
		}
	";

	let expected = [
		Token::new(TokenKind::Let, (2, 3)),
		Token::new("five", (2, 7)),
		Token::new(TokenKind::Assign, (2, 12)),
		Token::new(5, (2, 14)),
		Token::new(TokenKind::Semicolon, (2, 15)),
		Token::new(TokenKind::Let, (3, 3)),
		Token::new("ten", (3, 7)),
		Token::new(TokenKind::Assign, (3, 11)),
		Token::new(10, (3, 13)),
		Token::new(TokenKind::Semicolon, (3, 15)),
		Token::new(TokenKind::Let, (5, 3)),
		Token::new("add", (5, 7)),
		Token::new(TokenKind::Assign, (5, 11)),
		Token::new(TokenKind::Function, (5, 13)),
		Token::new(TokenKind::LeftParen, (5, 15)),
		Token::new("x", (5, 16)),
		Token::new(TokenKind::Comma, (5, 17)),
		Token::new("y", (5, 19)),
		Token::new(TokenKind::RightParen, (5, 20)),
		Token::new(TokenKind::LeftBrace, (5, 22)),
		Token::new("x", (6, 4)),
		Token::new(TokenKind::Plus, (6, 6)),
		Token::new("y", (6, 8)),
		Token::new(TokenKind::RightBrace, (7, 3)),
		Token::new(TokenKind::Semicolon, (7, 4)),
		Token::new(TokenKind::Let, (9, 3)),
		Token::new("result", (9, 7)),
		Token::new(TokenKind::Assign, (9, 14)),
		Token::new("add", (9, 16)),
		Token::new(TokenKind::LeftParen, (9, 19)),
		Token::new("five", (9, 20)),
		Token::new(TokenKind::Comma, (9, 24)),
		Token::new("ten", (9, 26)),
		Token::new(TokenKind::RightParen, (9, 29)),
		Token::new(TokenKind::Semicolon, (9, 30)),
		Token::new(TokenKind::Bang, (10, 3)),
		Token::new(TokenKind::Minus, (10, 4)),
		Token::new(TokenKind::Slash, (10, 5)),
		Token::new(TokenKind::Asterisk, (10, 6)),
		Token::new(5, (10, 7)),
		Token::new(TokenKind::Semicolon, (10, 8)),
		Token::new(5, (11, 3)),
		Token::new(TokenKind::LessThan, (11, 5)),
		Token::new(10, (11, 7)),
		Token::new(TokenKind::GreatherThan, (11, 10)),
		Token::new(5, (11, 12)),
		Token::new(TokenKind::Semicolon, (11, 13)),
		Token::new(TokenKind::If, (13, 3)),
		Token::new(TokenKind::LeftParen, (13, 6)),
		Token::new(5, (13, 7)),
		Token::new(TokenKind::LessThan, (13, 9)),
		Token::new(10, (13, 11)),
		Token::new(TokenKind::RightParen, (13, 13)),
		Token::new(TokenKind::LeftBrace, (13, 15)),
		Token::new(TokenKind::Return, (14, 4)),
		Token::new(TokenKind::True, (14, 11)),
		Token::new(TokenKind::Semicolon, (14, 15)),
		Token::new(TokenKind::RightBrace, (15, 3)),
		Token::new(TokenKind::Else, (15, 5)),
		Token::new(TokenKind::LeftBrace, (15, 10)),
		Token::new(TokenKind::Return, (16, 4)),
		Token::new(TokenKind::False, (16, 11)),
		Token::new(TokenKind::Semicolon, (16, 16)),
		Token::new(TokenKind::RightBrace, (17, 3)),
	];

	let tokenizer = Tokenizer::new(input);
	let mut i = 0;

	for token in tokenizer {
		assert_eq!(token?, expected[i], "Failed at #{i}");
		i += 1;
	}

	assert_eq!(i, expected.len());

	Ok(())
}

#[test]
fn even_more_operators() -> Result<()> {
	let input = r"
		let five = 5;
		let ten = 10;

		let add = fn(x, y) {
			x + y
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
	";

	let expected = [
		Token::new(TokenKind::Let, (2, 3)),
		Token::new("five", (2, 7)),
		Token::new(TokenKind::Assign, (2, 12)),
		Token::new(5, (2, 14)),
		Token::new(TokenKind::Semicolon, (2, 15)),
		Token::new(TokenKind::Let, (3, 3)),
		Token::new("ten", (3, 7)),
		Token::new(TokenKind::Assign, (3, 11)),
		Token::new(10, (3, 13)),
		Token::new(TokenKind::Semicolon, (3, 15)),
		Token::new(TokenKind::Let, (5, 3)),
		Token::new("add", (5, 7)),
		Token::new(TokenKind::Assign, (5, 11)),
		Token::new(TokenKind::Function, (5, 13)),
		Token::new(TokenKind::LeftParen, (5, 15)),
		Token::new("x", (5, 16)),
		Token::new(TokenKind::Comma, (5, 17)),
		Token::new("y", (5, 19)),
		Token::new(TokenKind::RightParen, (5, 20)),
		Token::new(TokenKind::LeftBrace, (5, 22)),
		Token::new("x", (6, 4)),
		Token::new(TokenKind::Plus, (6, 6)),
		Token::new("y", (6, 8)),
		Token::new(TokenKind::RightBrace, (7, 3)),
		Token::new(TokenKind::Semicolon, (7, 4)),
		Token::new(TokenKind::Let, (9, 3)),
		Token::new("result", (9, 7)),
		Token::new(TokenKind::Assign, (9, 14)),
		Token::new("add", (9, 16)),
		Token::new(TokenKind::LeftParen, (9, 19)),
		Token::new("five", (9, 20)),
		Token::new(TokenKind::Comma, (9, 24)),
		Token::new("ten", (9, 26)),
		Token::new(TokenKind::RightParen, (9, 29)),
		Token::new(TokenKind::Semicolon, (9, 30)),
		Token::new(TokenKind::Bang, (10, 3)),
		Token::new(TokenKind::Minus, (10, 4)),
		Token::new(TokenKind::Slash, (10, 5)),
		Token::new(TokenKind::Asterisk, (10, 6)),
		Token::new(5, (10, 7)),
		Token::new(TokenKind::Semicolon, (10, 8)),
		Token::new(5, (11, 3)),
		Token::new(TokenKind::LessThan, (11, 5)),
		Token::new(10, (11, 7)),
		Token::new(TokenKind::GreatherThan, (11, 10)),
		Token::new(5, (11, 12)),
		Token::new(TokenKind::Semicolon, (11, 13)),
		Token::new(TokenKind::If, (13, 3)),
		Token::new(TokenKind::LeftParen, (13, 6)),
		Token::new(5, (13, 7)),
		Token::new(TokenKind::LessThan, (13, 9)),
		Token::new(10, (13, 11)),
		Token::new(TokenKind::RightParen, (13, 13)),
		Token::new(TokenKind::LeftBrace, (13, 15)),
		Token::new(TokenKind::Return, (14, 4)),
		Token::new(TokenKind::True, (14, 11)),
		Token::new(TokenKind::Semicolon, (14, 15)),
		Token::new(TokenKind::RightBrace, (15, 3)),
		Token::new(TokenKind::Else, (15, 5)),
		Token::new(TokenKind::LeftBrace, (15, 10)),
		Token::new(TokenKind::Return, (16, 4)),
		Token::new(TokenKind::False, (16, 11)),
		Token::new(TokenKind::Semicolon, (16, 16)),
		Token::new(TokenKind::RightBrace, (17, 3)),
		Token::new(10, (19, 3)),
		Token::new(TokenKind::Equal, (19, 6)),
		Token::new(10, (19, 9)),
		Token::new(TokenKind::Semicolon, (19, 11)),
		Token::new(10, (20, 3)),
		Token::new(TokenKind::NotEqual, (20, 6)),
		Token::new(9, (20, 9)),
		Token::new(TokenKind::Semicolon, (20, 10)),
	];

	let tokenizer = Tokenizer::new(input);
	let mut i = 0;

	for token in tokenizer {
		assert_eq!(token?, expected[i], "Failed at #{i}");
		i += 1;
	}

	assert_eq!(i, expected.len());

	Ok(())
}
