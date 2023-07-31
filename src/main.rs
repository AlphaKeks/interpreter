use {
	clap::{Parser, ValueEnum},
	color_eyre::{eyre::Context, Result},
	std::io::{stdin, stdout, IsTerminal, Read, Write},
};

/// Monkey language interpreter!
#[derive(Debug, Parser)]
struct Args {
	/// Print debug logs
	#[arg(long)]
	debug: bool,

	/// Mode to run in
	#[arg(long)]
	#[clap(default_value = "tokenizer")]
	mode: Mode,
}

const PROMPT: &str = "=> ";

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Args::parse();

	if args.debug {
		monkey::tracing::setup();
	}

	match stdin().is_terminal() {
		true => match args.mode {
			Mode::Tokenizer => run_tokenizer().context("Tokenizer crashed"),
		},
		false => process_stdin(args.mode),
	}?;

	Ok(())
}

fn process_stdin(mode: Mode) -> Result<()> {
	let mut buf = Vec::new();
	stdin()
		.read_to_end(&mut buf)
		.context("Failed to read STDIN into buffer")?;

	let input = std::str::from_utf8(&buf).context("Input is not valid UTF-8")?;

	match mode {
		Mode::Tokenizer => {
			for token in monkey::Tokenizer::new(input.trim()) {
				println!("{:?}", token?);
			}
		}
	};

	Ok(())
}

fn run_tokenizer() -> Result<()> {
	loop {
		print!("[Tokenizer] {PROMPT}");
		stdout()
			.flush()
			.context("Failed to flush STDOUT")?;

		let mut input = String::new();
		stdin()
			.read_line(&mut input)
			.context("Failed to read from STDIN")?;

		for token in monkey::Tokenizer::new(input.trim()) {
			println!("{:?}", token?);
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Mode {
	Tokenizer,
}
