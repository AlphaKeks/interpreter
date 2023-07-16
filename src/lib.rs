pub mod tracing;

pub mod token;
pub use token::Token;

pub mod tokenizer;
pub use tokenizer::Tokenizer;

pub mod ast;
pub use ast::{Expression, Program, Statement};

pub mod parser;
pub use parser::Parser;

#[cfg(test)]
mod test_setup;

#[macro_export]
macro_rules! record {
	($name:expr, $thing:expr) => {{
		::tracing::Span::current().record($name, format_args!("{:?}", $thing));
		$thing
	}};
}
