use {
	clap::Parser,
	color_eyre::{eyre::Context, Result},
	interpreter::{Token, Tokenizer},
	std::{
		io::{stdin, stdout, Write},
		time::Instant,
	},
};

#[derive(Debug, Parser)]
pub struct Args {
	/// Print debug information
	#[arg(long)]
	pub debug: bool,
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
		let mut tokenizer = Tokenizer::new(input.chars().collect());
		let mut token = tokenizer.next_token()?;

		let mut start_token = Instant::now();
		while token != Token::Eof {
			println!("{token:?} ({:?})", start_token.elapsed());
			token = tokenizer.next_token()?;
			start_token = Instant::now();
		}

		println!("took {:?}", start.elapsed());
	}

	Ok(())
}
