use std::rc::Rc;

use log::error;
use once_cell::unsync::Lazy;
use rusqlite::Connection;

use crate::{
	error::{IError, IResult},
	gui::ConnConf,
	util::app_db_file,
};

const APP_TABLES: [(&'static str, &'static str); 1] = [(
	"t_conn_conf",
	"CREATE TABLE t_conn_conf (
		uuid  TEXT PRIMARY KEY,
		type  TEXT NOT NULL,
		url  TEXT NOT NULL,
		username  TEXT,
		password  TEXT
	)",
)];

//? Err will be cached in local thread?
pub fn get_conn() -> IResult<Rc<Connection>> {
	thread_local! {
		static CONN: Lazy<IResult<Rc<Connection>>> = Lazy::new(|| {
			let conn = Connection::open(app_db_file().as_path().clone())?;
			Ok(Rc::new(conn))
		})
	}

	CONN.with(|conn| match conn.as_ref() {
		Ok(rc) => Ok(rc.clone()),
		Err(e) => {
			error!("Create connection failed, Caused by: {}", e);
			Err(IError::PromptError("Create connection failed".to_string()))
		}
	})
}

pub fn save_conn_conf(conf: &ConnConf) -> IResult<()> {
	let conn = get_conn()?;
	let uuid = uuid::Uuid::new_v4().to_string();
	conn.execute(
		"INSERT INTO t_conn_conf(uuid, type url, username, password) VALUES (?1, ?2, ?3, ?4, ?5)",
		(&uuid, &conf.db_type, &conf.url, &conf.username, &conf.password),
	)?;
	Ok(())
}

/// Create tables if necessary.
pub fn init_db_if_needed() -> IResult<()> {
	let conn = get_conn()?;

	let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
	let existing_tables = stmt
		.query_map([], |row| Ok(row.get::<usize, String>(0)?))?
		.map(|row| row.unwrap_or("".to_string()))
		.collect::<Vec<_>>();

	for table in APP_TABLES {
		if !existing_tables.contains(&table.0.to_string()) {
			conn.execute(table.1, ())?;
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {}
