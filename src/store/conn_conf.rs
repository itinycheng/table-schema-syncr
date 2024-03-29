use uuid::Uuid;
use validator::Validate;

use crate::{error::IResult, mapping::database::DbType};

use super::get_conn;

#[derive(Debug, Default, Validate, Clone)]
pub struct ConnConf {
	pub uuid: String,
	#[validate(length(min = 1))]
	pub name: String,
	#[validate(required)]
	pub db_type: Option<DbType>,
	#[validate(length(min = 1))]
	pub url: String,
	#[validate(length(min = 1))]
	pub username: String,
	pub password: String,
}

pub fn insert_or_update(conf: &ConnConf) -> IResult<()> {
	if conf.uuid.is_empty() {
		insert(conf)
	} else {
		update(conf)
	}
}

pub fn insert(conf: &ConnConf) -> IResult<()> {
	conf.validate()?;

	let conn = get_conn();
	let uuid = Uuid::new_v4().to_string();
	conn.execute(
		"INSERT INTO t_conn_conf(uuid, name, type, url, username, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
		(
			&uuid,
			&conf.name,
			&conf.db_type.as_ref().map(|db_ty| db_ty.to_string()).unwrap(),
			&conf.url,
			&conf.username,
			&conf.password,
		),
	)?;

	Ok(())
}

pub fn update(conf: &ConnConf) -> IResult<()> {
	conf.validate()?;

	let conn = get_conn();
	conn.execute(
		"UPDATE t_conn_conf set name=?1, type=?2, url=?3, username=?4, password=?5 WHERE uuid = ?6",
		(
			&conf.name,
			&conf.db_type.as_ref().map(|db_ty| db_ty.to_string()).unwrap(),
			&conf.url,
			&conf.username,
			&conf.password,
			&conf.uuid,
		),
	)?;

	Ok(())
}

pub fn delete(uuid: &String) -> IResult<()> {
	let conn = get_conn();
	conn.execute("DELETE FROM t_conn_conf WHERE uuid = ?1", (uuid,))?;
	Ok(())
}

pub fn query_by_uuid(uuid: &String) -> IResult<ConnConf> {
	let conn = get_conn();
	Ok(conn.query_row(
		"SELECT uuid, name, type, url, username, password FROM t_conn_conf where uuid = ?1 ",
		[uuid],
		|row| {
			Ok(ConnConf {
				uuid: row.get(0)?,
				name: row.get(1)?,
				db_type: Some(row.get::<usize, String>(2)?.into()),
				url: row.get(3)?,
				username: row.get(4)?,
				password: row.get(5)?,
			})
		},
	)?)
}

pub fn list_all() -> IResult<Vec<ConnConf>> {
	let conn = get_conn();
	let mut stmt =
		conn.prepare("SELECT uuid, name, type, url, username, password FROM t_conn_conf")?;
	let list = stmt
		.query_map((), |row| {
			Ok(ConnConf {
				uuid: row.get(0)?,
				name: row.get(1)?,
				db_type: Some(row.get::<usize, String>(2)?.into()),
				url: row.get(3)?,
				username: row.get(4)?,
				password: row.get(5)?,
			})
		})?
		.map(|item| item.unwrap())
		.collect::<Vec<_>>();

	Ok(list)
}
