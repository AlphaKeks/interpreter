use {
	clap::{Parser as _, ValueEnum},
	color_eyre::{eyre::Context, Result},
	monkey::{eval::Environment, Eval, Parser, Token, Tokenizer},
	std::{
		io::{stdin, stdout, Write},
		rc::Rc,
		time::Instant,
	},
};

#[derive(Debug, clap::Parser)]
pub struct Args {
	/// Print debug information
	#[arg(long)]
	debug: bool,

	#[arg(long)]
	#[clap(default_value = "interpreter")]
	mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Mode {
	Tokenizer,
	Parser,
	Interpreter,
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

const PROMPT: &str = "=> ";

fn main() -> Result<()> {
	color_eyre::install()?;

	let Args { debug, mode } = Args::parse();
	if debug {
		monkey::tracing::init();
	}

	println!("Hello! This is the Monkey programming language!");
	println!("Feel free to type in commands.");
	println!("You can type `quit` to quit.");

	match mode {
		Mode::Tokenizer => tokenize(mode),
		Mode::Parser => parse(mode),
		Mode::Interpreter => interpret(mode),
	}?;

	Ok(())
}

fn tokenize(mode: Mode) -> Result<()> {
	loop {
		print!("[{mode:?}] {PROMPT}");
		stdout()
			.flush()
			.context("Failed to flush STDOUT")?;

		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.context("Failed to read from STDIN")?;

		if matches!(input.trim(), "quit" | "exit" | "bye") {
			println!("Bye.");
			break;
		}

		let input = input.chars().collect();
		let mut tokenizer = Tokenizer::new(input);
		let mut token = tokenizer.step()?;

		let mut start = Instant::now();
		while token != Token::Eof {
			println!("{token:?} ({:?})", start.elapsed());
			token = tokenizer.step()?;
			start = Instant::now();
		}
	}

	Ok(())
}

fn parse(mode: Mode) -> Result<()> {
	loop {
		print!("[{mode:?}] {PROMPT}");
		stdout()
			.flush()
			.context("Failed to flush STDOUT")?;

		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.context("Failed to read from STDIN")?;

		if matches!(input.trim(), "quit" | "exit" | "bye") {
			println!("Bye.");
			break;
		}

		let input = input.chars().collect();
		let tokenizer = Tokenizer::new(input);
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();

		if !parser.errors.is_empty() {
			eprintln!("Failed to parse.\n{:?}", parser.errors);
			continue;
		}

		println!("{program:#?}");
	}

	Ok(())
}

fn interpret(mode: Mode) -> Result<()> {
	let mut environment = Rc::new(Environment::new_global());

	loop {
		print!("[{mode:?}] {PROMPT}");
		stdout()
			.flush()
			.context("Failed to flush STDOUT")?;

		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.context("Failed to read from STDIN")?;

		match input.trim() {
			"reset" => {
				environment = Rc::new(Environment::new_global());
				println!("Environment has been reset.");
				continue;
			}
			"quit" | "exit" | "bye" => {
				println!("Bye.");
				break;
			}

			_ => {}
		};

		let input = input.chars().collect();
		let tokenizer = Tokenizer::new(input);
		let mut parser = Parser::new(tokenizer)?;
		let program = parser.parse_program();

		if !parser.errors.is_empty() {
			eprintln!("Failed to parse.\n{:?}", parser.errors);
			continue;
		}

		match program.eval(Rc::clone(&environment)) {
			Ok(evaluated) => println!("{evaluated}"),
			Err(error) => eprintln!("{error}"),
		};
	}

	Ok(())
}
