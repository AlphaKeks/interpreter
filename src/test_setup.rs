#[ctor::ctor]
fn test_setup() {
	color_eyre::install().expect("Failed to setup color-eyre");
	crate::tracing::setup();
}
