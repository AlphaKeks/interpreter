#[ctor::ctor]
fn setup() {
	color_eyre::install().expect("Failed to setup color-eyre");
	crate::tracing::init();
}
