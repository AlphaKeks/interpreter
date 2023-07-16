use crate::Statement;

#[derive(Debug, Default)]
pub struct Program {
	pub(crate) statements: Vec<Statement>,
}

impl std::fmt::Display for Program {
	#[tracing::instrument(level = "TRACE", skip(f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let program = self
			.statements
			.iter()
			.map(|statement| statement.to_string())
			.collect::<Vec<_>>()
			.join("");

		write!(f, "{program}")
	}
}
