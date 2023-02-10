use std::{
	fs::{self, File},
	io,
	path::{Path, PathBuf},
};

use directories::UserDirs;

const APP_ROOT_PATH: &'static str = ".schema_syncr";

const APP_DATA_FILE: &'static str = "data.db";

const APP_LOG_FILE: &'static str = "logs/app.log";

pub fn app_root_dir() -> PathBuf {
	UserDirs::new().unwrap().home_dir().join(APP_ROOT_PATH)
}

pub fn app_db_file() -> PathBuf {
	app_root_dir().join(APP_DATA_FILE)
}

pub fn app_log_file() -> PathBuf {
	app_root_dir().join(APP_LOG_FILE)
}

pub fn exists(path: &Path) -> bool {
	Path::new(path).exists()
}

pub fn create_not_exist_file(path: &Path) -> io::Result<()> {
	if !exists(path) {
		let _ = create_file(&path);
	}

	Ok(())
}

fn create_file(path: &Path) -> io::Result<File> {
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent)?
	}

	let file = File::create(path)?;
	Ok(file)
}
