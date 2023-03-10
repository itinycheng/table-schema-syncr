use std::path::PathBuf;

use chrono::Local;
use log::info;
use tracing::Level;
use tracing_appender::rolling;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::time::FormatTime, EnvFilter};

use crate::{store, util};

pub(crate) fn app_init() {
	let log_file = util::app_log_file();
	util::create_if_not_exist(log_file).unwrap();
	init_logger(log_file);

	let db_file = util::app_db_file();
	util::create_if_not_exist(db_file).unwrap();
	store::init_db_if_needed().unwrap();
}

fn init_logger(log_file: &PathBuf) {
	LogTracer::init().expect("Failed to set logger");

	let log_path = log_file.parent().unwrap();
	let file = log_file.file_name().unwrap();
	let file_appender = rolling::daily(log_path, file);

	let subscriber = tracing_subscriber::fmt()
		.with_timer(LogFormatter)
		.with_writer(file_appender)
		.with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
		.finish();

	tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

	info!("Use tracing as backend of logger!");
}
struct LogFormatter;

impl FormatTime for LogFormatter {
	fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
		write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
	}
}
