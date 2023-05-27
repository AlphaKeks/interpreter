use {
	time::macros::format_description,
	tracing::trace,
	tracing_subscriber::{
		fmt::{format::FmtSpan, time::UtcTime},
		EnvFilter,
	},
};

pub fn init() {
	let timer = UtcTime::new(format_description!(
		"[[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]]"
	));

	let filter =
		EnvFilter::try_from_default_env().unwrap_or_else(|_| "WARN,interpreter=DEBUG".into());

	tracing_subscriber::fmt()
		.pretty()
		.with_timer(timer)
		.with_file(true)
		.with_line_number(true)
		.with_span_events(FmtSpan::ACTIVE)
		.with_env_filter(filter)
		.init();

	trace!("Initialized logging");
}
