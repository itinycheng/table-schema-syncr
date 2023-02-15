use super::{ClickHouseRow, DBClient, DBParam, DBQuery};
use crate::{
	database::DB_CLICK_HOUSE,
	error::{IError, IResult},
};
use clickhouse::{Client, Compression};
use serde::Deserialize;
use std::sync::Arc;

pub(super) fn create_ch_client(ds: &DBParam) -> IResult<DBClient> {
	let mut client = Client::default()
		.with_url(&ds.url)
		.with_database(&ds.database)
		.with_user(&ds.user)
		.with_password(&ds.password)
		.with_compression(to_compression(&ds.compression));
	for (key, value) in &ds.options {
		client = client.with_option(key, value);
	}

	if tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()?
		.block_on(client.query("select 1").fetch_one::<u8>())
		.is_err()
	{
		return Err(IError::PromptError(format!("Ping ClickHouse failed, uuid: {}", ds.uuid)));
	}

	Ok(DBClient::ClickHouse(Arc::new(client)))
}

impl<T: ClickHouseRow + for<'a> Deserialize<'a>> DBQuery<DB_CLICK_HOUSE, T> for DBClient {
	fn query_list<I: AsRef<str>>(&self, sql: I) -> IResult<Vec<T>> {
		match self {
			DBClient::ClickHouse(client) => {
				let rows = tokio::runtime::Builder::new_current_thread()
					.enable_all()
					.build()?
					.block_on(client.query(sql.as_ref()).fetch_all::<T>())?;
				Ok(rows)
			}
			_ => unreachable!(),
		}
	}

	fn query_one<I: AsRef<str>>(&self, sql: I) -> IResult<Option<T>> {
		match self {
			DBClient::ClickHouse(client) => {
				let result = tokio::runtime::Builder::new_current_thread()
					.enable_all()
					.build()?
					.block_on(client.query(sql.as_ref()).fetch_one::<T>());

				match result {
					Ok(row) => Ok(Some(row)),
					Err(clickhouse::error::Error::RowNotFound) => Ok(None),
					Err(e) => Err(e)?,
				}
			}
			_ => unreachable!(),
		}
	}
}

fn to_compression(str: &String) -> Compression {
	match &*str.to_lowercase() {
		"Lz4" => Compression::Lz4,
		"Lz4Hc(1)" => Compression::Lz4Hc(1),
		"Lz4Hc(4)" => Compression::Lz4Hc(4),
		"Lz4Hc(9)" => Compression::Lz4Hc(9),
		"Lz4Hc(12)" => Compression::Lz4Hc(12),
		_ => Compression::None,
	}
}
