use std::{
	borrow::Borrow,
	collections::HashMap,
	hash::{Hash, Hasher},
	sync::Arc,
};

use ::clickhouse::Client;
use ::mysql::Pool;
use serde::{Deserialize, Serialize};

use crate::{database::DbType, error::IResult};

pub use self::{
	clickhouse::{ClickHouseClient, ClickHouseRow},
	mysql::{MysqlClient, MysqlRow},
};

mod clickhouse;
mod mysql;

#[derive(Clone)]
pub enum DBClient<const DB: u8> {
	ClickHouse(Arc<Client>),
	Mysql(Pool),
}

pub trait DBCreator<const DB: u8> {
	const DB_TYPE: DbType = DbType::to_enum(DB);

	fn get_or_init(ds: DsParam) -> IResult<DBClient<DB>>;

	fn get_by_uuid<T: Borrow<String>>(t: &T) -> Option<DBClient<DB>>;
}

pub trait DBAccessor<T> {
	fn query_list<I: AsRef<str>>(&self, sql: I) -> IResult<Vec<T>>;

	fn query_one<I: AsRef<str>>(&self, sql: I) -> IResult<Option<T>>;
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DsParam {
	pub uuid: String,
	pub url: String,
	pub database: String,
	pub user: String,
	pub password: String,
	pub compression: String,
	pub options: HashMap<String, String>,
}

impl Hash for DsParam {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.uuid.hash(state);
	}
}

impl Borrow<String> for DsParam {
	fn borrow(&self) -> &String {
		&self.uuid
	}
}
