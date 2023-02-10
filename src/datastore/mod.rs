use std::fs;

use directories::UserDirs;
use rusqlite::Connection;

use crate::{error::IResult};

#[derive(Debug)]
struct Person {
	id: i32,
	name: String,
	data: Option<Vec<u8>>,
}

pub fn test() -> IResult<()> {
	if let Some(user_dir) = UserDirs::new() {
		let app_path = user_dir.home_dir().join("APP_ROOT_PATH");
		let _ = fs::create_dir(app_path.clone());
		let conn = Connection::open(app_path.join("APP_DATA_FILE"))?;

		conn.execute(
			"CREATE TABLE person (
				id    INTEGER PRIMARY KEY,
				name  TEXT NOT NULL,
				data  BLOB
			)",
			(),
		)?;

		let me = Person { id: 0, name: "Steven".to_string(), data: None };
		conn.execute("INSERT INTO person (name, data) VALUES (?1, ?2)", (&me.name, &me.data))?;

		let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
		let person_iter = stmt.query_map([], |row| {
			Ok(Person { id: row.get(0)?, name: row.get(1)?, data: row.get(2)? })
		})?;

		for person in person_iter {
			println!("Found person {:?}", person.unwrap());
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests {

	use super::test;

	#[test]
	fn test_create_table() {
		let a = test();
		println!("{:?}", a);
	}
}
