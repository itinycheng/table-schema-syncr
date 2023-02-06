use std::{
	borrow::Borrow,
	collections::HashMap,
	hash::{Hash, Hasher},
	sync::{Arc, RwLock},
};
use ::clickhouse::Client;
use ::mysql::Pool;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{
	database::DbType,
	error::{IError, IResult},
};

pub use ::clickhouse::Row as ClickHouseRow;
pub use ::mysql::prelude::FromRow as MysqlRow;

mod clickhouse;
mod mysql;

static DB_CLIENTS: Lazy<RwLock<HashMap<DBParam, DBClient>>> =
	Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone)]
pub enum DBClient {
	ClickHouse(Arc<Client>),
	Mysql(Pool),
}

impl DBClient {
	pub fn get_or_init(ds: DBParam) -> IResult<DBClient> {
		match Self::get(&ds) {
			Some(cli) => Ok(cli.clone()),
			None => {
				let client = match &ds.db_type {
					&DbType::MySQL => mysql::create_mysql_client(&ds),
					&DbType::ClickHouse => clickhouse::create_ch_client(&ds),
					_ => {
						Err(IError::PromptError(format!("Unsupported db type: {:?}", &ds.db_type)))
					}
				};

				if client.is_ok() {
					let mut write_lock = DB_CLIENTS.write().unwrap();
					write_lock.insert(ds, client.as_ref().unwrap().clone());
				}

				client
			}
		}
	}

	pub fn get<T: Borrow<String>>(t: &T) -> Option<DBClient> {
		let read_lock = DB_CLIENTS.read().unwrap();
		read_lock.get(t.borrow()).map(|pool| pool.clone())
	}
}

pub trait DBQuery<const DB: u8, T> {
	fn query_list<I: AsRef<str>>(&self, sql: I) -> IResult<Vec<T>>;

	fn query_one<I: AsRef<str>>(&self, sql: I) -> IResult<Option<T>>;
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default, Clone)]
pub struct DBParam {
	pub uuid: String,
	pub db_type: DbType,
	pub url: String,
	pub database: String,
	pub user: String,
	pub password: String,
	pub compression: String,
	pub options: HashMap<String, String>,
}

impl Hash for DBParam {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.uuid.hash(state);
	}
}

impl Borrow<String> for DBParam {
	fn borrow(&self) -> &String {
		&self.uuid
	}
}
