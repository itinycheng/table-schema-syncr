use std::rc::Rc;

use once_cell::unsync::Lazy;
use rusqlite::Connection;

use crate::{error::IResult, util::app_db_file};

pub mod conn_conf;

const APP_TABLES: [(&'static str, &'static str); 1] = [(
	"t_conn_conf",
	"CREATE TABLE t_conn_conf (
		uuid  TEXT PRIMARY KEY,
		name TEXT NOT NULL,
		type  TEXT NOT NULL,
		url  TEXT NOT NULL,
		username  TEXT,
		password  TEXT
	)",
)];

//? If return type is something like `Result<>`, err may be cached?
pub fn get_conn<'a>() -> Rc<Connection> {
	thread_local! {
		static CONN: Lazy<Rc<Connection>> = Lazy::new(|| {
			Rc::new(Connection::open(app_db_file().as_path().clone()).unwrap())
		})
	}

	CONN.with(|conn| (*conn).clone())
}

/// Create tables if necessary.
pub fn init_db_if_needed() -> IResult<()> {
	let conn = get_conn();

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
mod tests {
	use std::{
		rc::Rc,
		time::{SystemTime, UNIX_EPOCH},
	};

	use once_cell::unsync::Lazy;

	use crate::error::{IError, IResult};

	#[test]
	fn test_thread_local() {
		thread_local! {
			static CONN: Lazy<IResult<Rc<i32>>> = Lazy::new(|| {
				let timestamp = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.unwrap()
					.as_secs();
				Err(IError::PromptError(format!("error: {}", timestamp)))
			})
		}

		for i in 0..5 {
			CONN.with(|res| match res.as_ref() {
				Ok(value) => println!("i = {}, {:?}", i, value),
				Err(e) => println!("{:?}", e),
			});
		}
	}
}
