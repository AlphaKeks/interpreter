pub mod tracing;

pub mod token;
pub use token::Token;

pub mod tokenizer;
pub use tokenizer::Tokenizer;

#[macro_export]
macro_rules! record {
	($name:expr, $thing:expr) => {{
		::tracing::Span::current().record($name, format_args!("{:?}", $thing));
		$thing
	}};
}
