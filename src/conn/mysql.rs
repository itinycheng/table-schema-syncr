use std::{borrow::Borrow, collections::HashMap, sync::RwLock};

use mysql::{
	prelude::{FromRow, Queryable},
	Opts, Pool,
};
use once_cell::sync::Lazy;

use crate::{database::DB_MYSQL, error::IResult};

use super::{DBAccessor, DBClient, DBCreator, DsParam};

static CLIENTS: Lazy<RwLock<HashMap<DsParam, DBClient<DB_MYSQL>>>> =
	Lazy::new(|| RwLock::new(HashMap::new()));

impl DBCreator<DB_MYSQL> for DBClient<DB_MYSQL> {
	fn get_or_init(ds: DsParam) -> IResult<DBClient<DB_MYSQL>> {
		let read_lock = CLIENTS.read().unwrap();
		match read_lock.get(&ds) {
			Some(pool) => Ok(pool.clone()),
			None => {
				drop(read_lock);

				let opts = Opts::from_url(&ds.url)?;
				let pool = Pool::new(opts)?;
				let db_pool = DBClient::<DB_MYSQL>::Mysql(pool);
				let cloned = db_pool.clone();
				let mut write_lock = CLIENTS.write().unwrap();
				write_lock.insert(ds, db_pool);

				Ok(cloned)
			}
		}
	}

	fn get_by_uuid<T: Borrow<String>>(t: &T) -> Option<DBClient<DB_MYSQL>> {
		let read_lock = CLIENTS.read().unwrap();
		read_lock.get(t.borrow()).map(|pool| pool.clone())
	}
}

impl<T: FromRow> DBAccessor<T> for DBClient<DB_MYSQL> {
	fn query_list<I: AsRef<str>>(&self, sql: I) -> IResult<Vec<T>> {
		match self {
			DBClient::Mysql(pool) => {
				let mut conn = pool.get_conn()?;
				let res = conn.query::<T, I>(sql)?;
				Ok(res)
			}
			_ => unreachable!(),
		}
	}

	fn query_one<I: AsRef<str>>(&self, sql: I) -> IResult<Option<T>> {
		match self {
			DBClient::Mysql(pool) => {
				let mut conn = pool.get_conn()?;
				let row = conn.query_first::<T, I>(sql)?;
				Ok(row)
			}
			_ => unreachable!(),
		}
	}
}
