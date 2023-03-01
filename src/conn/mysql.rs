use mysql::{prelude::Queryable, Opts, Pool};

use crate::{
	database::DbType,
	error::{IError, IResult},
};

use super::{DBClient, DBParam, DBQuery, MysqlRow};

pub(super) fn create_mysql_client(ds: &DBParam) -> IResult<DBClient> {
	let opts = Opts::from_url(&ds.url)?;
	let pool = Pool::new(opts)?;

	{
		let mut conn = pool.get_conn()?;
		if !conn.as_mut().ping() {
			return Err(IError::PromptError(format!("Ping mysql failed, uuid: {}", ds.uuid)));
		}
	}

	Ok(DBClient::Mysql(pool))
}

impl<T: MysqlRow> DBQuery<{ DbType::DB_MYSQL }, T> for DBClient {
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
