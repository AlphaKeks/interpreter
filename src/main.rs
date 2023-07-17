use {
	clap::{Parser as _, ValueEnum},
	color_eyre::{eyre::Context, Result},
	interpreter::{Parser, Token, Tokenizer},
	std::{
		io::{stdin, stdout, Write},
		time::Instant,
	},
};

#[derive(Debug, clap::Parser)]
pub struct Args {
	/// Print debug information
	#[arg(long)]
	debug: bool,

	#[arg(long)]
	#[clap(default_value = "parser")]
	mode: Mode,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
	Tokenizer,
	Parser,
}

impl std::fmt::Display for Mode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

const PROMPT: &str = "=> ";

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Args::parse();
	if args.debug {
		interpreter::tracing::init();
	}

	println!("Hello! This is the Monkey programming language!");
	println!("Feel free to type in commands.");
	println!("You can type `quit` to quit.");

	loop {
		print!("{PROMPT}");
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

		let start = Instant::now();
		let input = input.chars().collect();
		match args.mode {
			Mode::Tokenizer => {
				let mut tokenizer = Tokenizer::new(input);
				let mut token = tokenizer.step()?;

				let mut start_token = Instant::now();
				while token != Token::Eof {
					println!("{token:?} ({:?})", start_token.elapsed());
					token = tokenizer.step()?;
					start_token = Instant::now();
				}
			}

			Mode::Parser => {
				let tokenizer = Tokenizer::new(input);
				let mut parser = Parser::new(tokenizer)?;
				let program = parser.parse_program();

				if !parser.errors.is_empty() {
					eprintln!("Failed to parse.\n{:?}", parser.errors);
					continue;
				}

				println!("{program:#?}");
			}
		};

		println!("took {:?}", start.elapsed());
	}

	Ok(())
}
