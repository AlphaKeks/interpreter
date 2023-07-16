use {
	crate::{Program, Statement},
	color_eyre::Result,
	pretty_assertions::assert_eq,
};

#[test]
fn test_print() -> Result<()> {
	let program = Program { statements: vec![Statement::r#let("my_var", "another_var")] };
	let expected = "let my_var = another_var;";

	assert_eq!(program.to_string(), expected);

	Ok(())
}
