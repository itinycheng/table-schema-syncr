use std::{
	borrow::Borrow,
	collections::HashMap,
	sync::{Arc, RwLock},
};

use super::{DBAccessor, DBClient, DBCreator, DsParam};
use crate::{database::DB_CLICK_HOUSE, error::IResult};
use clickhouse::{Client, Compression};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub use clickhouse::Row as ClickHouseRow;

static CLIENTS: Lazy<RwLock<HashMap<DsParam, DBClient<DB_CLICK_HOUSE>>>> =
	Lazy::new(|| RwLock::new(HashMap::new()));

impl DBCreator<DB_CLICK_HOUSE> for DBClient<DB_CLICK_HOUSE> {
	fn get_or_init(ds: DsParam) -> IResult<DBClient<DB_CLICK_HOUSE>> {
		let read_lock = CLIENTS.read().unwrap();
		match read_lock.get(&ds) {
			Some(cli) => Ok(cli.clone()),
			None => {
				drop(read_lock);

				let mut client = Client::default()
					.with_url(&ds.url)
					.with_database(&ds.database)
					.with_user(&ds.user)
					.with_password(&ds.password)
					.with_compression(to_compression(&ds.compression));
				for (key, value) in &ds.options {
					client = client.with_option(key, value);
				}

				let ref_pool = DBClient::<DB_CLICK_HOUSE>::ClickHouse(Arc::new(client));
				let cloned = ref_pool.clone();
				let mut write_lock = CLIENTS.write().unwrap();
				write_lock.insert(ds, ref_pool);

				Ok(cloned)
			}
		}
	}

	fn get_by_uuid<T: Borrow<String>>(t: &T) -> Option<DBClient<DB_CLICK_HOUSE>> {
		let read_lock = CLIENTS.read().unwrap();
		read_lock.get(t.borrow()).map(|pool| pool.clone())
	}
}

impl<T: ClickHouseRow + for<'a> Deserialize<'a>> DBAccessor<T> for DBClient<DB_CLICK_HOUSE> {
	fn query_list<I: AsRef<str>>(&self, sql: I) -> IResult<Vec<T>> {
		match self {
			DBClient::ClickHouse(client) => {
				let rows = tokio::runtime::Builder::new_current_thread()
					.enable_all()
					.build()
					.unwrap()
					.block_on(async { client.query(sql.as_ref()).fetch_all::<T>().await })?;
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
					.build()
					.unwrap()
					.block_on(async { client.query(sql.as_ref()).fetch_one::<T>().await });

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
