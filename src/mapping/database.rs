use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum DbType {
	MySQL = 1,
	ClickHouse = 2,
	HBase = 3,
	#[default]
	Unknown = 0,
}

impl DbType {
	pub const ALL: &'static [DbType; 3] = &[DbType::MySQL, DbType::ClickHouse, DbType::HBase];

	pub const DB_MYSQL: u8 = DbType::MySQL as u8;

	pub const DB_CLICK_HOUSE: u8 = DbType::ClickHouse as u8;

	pub const DB_HBASE: u8 = DbType::HBase as u8;
}

impl std::fmt::Display for DbType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				DbType::MySQL => "MySQL",
				DbType::ClickHouse => "ClickHouse",
				DbType::HBase => "HBase",
				_ => "Unknown",
			}
		)
	}
}

impl From<String> for DbType {
	fn from(value: String) -> Self {
		match value.as_ref() {
			"MySQL" => Self::MySQL,
			"ClickHouse" => Self::ClickHouse,
			"HBase" => Self::HBase,
			_ => Self::Unknown,
		}
	}
}

impl From<u8> for DbType {
	fn from(value: u8) -> Self {
		match value {
			Self::DB_MYSQL => Self::MySQL,
			Self::DB_CLICK_HOUSE => Self::ClickHouse,
			Self::DB_HBASE => Self::HBase,
			_ => panic!("Unknown enum value"),
		}
	}
}
