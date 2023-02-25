use std::{
	fs::{self, File},
	io,
	path::{Path, PathBuf},
};

use directories::UserDirs;
use once_cell::sync::OnceCell;

const APP_ROOT_PATH: &'static str = ".schema_syncr";

const APP_DATA_FILE: &'static str = "data.db";

const APP_LOG_FILE: &'static str = "logs/app.log";

pub fn app_root_dir() -> &'static PathBuf {
	static ROOT_DIR: OnceCell<PathBuf> = OnceCell::new();
	ROOT_DIR.get_or_init(|| UserDirs::new().unwrap().home_dir().join(APP_ROOT_PATH))
}

pub fn app_db_file() -> &'static PathBuf {
	static DB_FILE: OnceCell<PathBuf> = OnceCell::new();
	DB_FILE.get_or_init(|| app_root_dir().join(APP_DATA_FILE))
}

pub fn app_log_file() -> &'static PathBuf {
	static LOG_FILE: OnceCell<PathBuf> = OnceCell::new();
	LOG_FILE.get_or_init(|| app_root_dir().join(APP_LOG_FILE))
}

pub fn exists(path: &Path) -> bool {
	Path::new(path).exists()
}

pub fn create_if_not_exist(path: &Path) -> io::Result<()> {
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
