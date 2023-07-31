use tracing_subscriber::EnvFilter;

pub fn setup() {
	tracing_subscriber::fmt()
		.pretty()
		.without_time()
		.with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "INFO".into()))
		.init();
}
