use ::clickhouse::Client;
use ::mysql::Pool;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
	borrow::Borrow,
	collections::HashMap,
	hash::{Hash, Hasher},
	sync::{Arc, RwLock},
};

use crate::{
	database::DbType,
	error::{IError, IResult},
	store::conn_conf::ConnConf,
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

	pub fn databases(&self) -> IResult<Vec<String>> {
		match self {
			DBClient::ClickHouse(_) => {
				DBQuery::<{ DbType::DB_CLICK_HOUSE }, String>::query_list(self, "show databases")
			}
			DBClient::Mysql(_) => {
				DBQuery::<{ DbType::DB_MYSQL }, String>::query_list(self, "show databases")
			}
		}
	}

	pub fn tables(&self, database: &String) -> IResult<Vec<String>> {
		match self {
			DBClient::ClickHouse(_) => DBQuery::<{ DbType::DB_CLICK_HOUSE }, String>::query_list(
				self,
				format!("show tables in {}", database),
			),
			DBClient::Mysql(_) => DBQuery::<{ DbType::DB_MYSQL }, String>::query_list(
				self,
				format!(
					"SELECT table_name FROM information_schema.tables WHERE table_schema = '{}'",
					database
				),
			),
		}
	}

	pub fn table_schema(&self, database: &String, table: &String) -> IResult<String> {
		match self {
			DBClient::ClickHouse(_) => DBQuery::<{ DbType::DB_CLICK_HOUSE }, String>::query_one(
				self,
				format!("show create table {}.{}", database, table),
			)
			.map(|schema| schema.unwrap_or("".to_owned())),
			DBClient::Mysql(_) => DBQuery::<{ DbType::DB_MYSQL }, (String, String)>::query_one(
				self,
				format!("show create table {}.{}", database, table),
			)
			.map(|schema| schema.map(|tpl| tpl.1).unwrap_or("".to_owned())),
		}
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

impl TryFrom<ConnConf> for DBParam {
	type Error = IError;
	fn try_from(value: ConnConf) -> Result<Self, Self::Error> {
		Ok(DBParam {
			uuid: value.uuid,
			db_type: value
				.db_type
				.ok_or(IError::PromptError("db type can't be None".to_string()))?,
			url: value.url,
			user: value.username,
			password: value.password,
			..Default::default()
		})
	}
}
