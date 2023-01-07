use serde::{Deserialize, Serialize};

pub const DB_MYSQL: u8 = DbType::MySQL as u8;
pub const DB_CLICK_HOUSE: u8 = DbType::ClickHouse as u8;
pub const DB_HBASE: u8 = DbType::HBase as u8;

#[repr(u8)]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum DbType {
	MySQL = 1,
	ClickHouse = 2,
	HBase = 3,
}

impl DbType {
	pub const fn to_enum(value: u8) -> Self {
		match value {
			DB_MYSQL => Self::MySQL,
			DB_CLICK_HOUSE => Self::ClickHouse,
			DB_HBASE => Self::HBase,
			_ => panic!("Unknown enum value"),
		}
	}
}
