use {clap::Parser, color_eyre::Result};

#[derive(Debug, Parser)]
pub struct Args {
	/// Print debug information
	#[arg(long)]
	pub debug: bool,
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Args::parse();
	if args.debug {
		interpreter::tracing::init();
	}

	println!("Hello, world!");

	Ok(())
}
