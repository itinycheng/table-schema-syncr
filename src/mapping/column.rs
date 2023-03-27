use crate::error::IResult;

use super::{database::DbType, type_parser_ch, type_parser_hbase, type_parser_mysql};

#[derive(Debug, Default, Clone)]
pub struct ColumnSpec {
	pub name: String,
	pub r#type: DataType,
	pub comment: String,
}

impl ColumnSpec {
	pub fn create(
		name: String,
		type_str: String,
		comment: String,
		db_type: DbType,
	) -> IResult<ColumnSpec> {
		Ok(ColumnSpec { name: name, r#type: DataType::parse(type_str, db_type)?, comment })
	}
}

#[derive(Debug, Default, Clone)]
pub enum DataType {
	Int(usize),
	UInt(usize),
	Float(usize),
	Decimal {
		precision: u8,
		scale: u8,
	},
	Bool,
	String(Option<usize>),
	Uuid,
	Date,
	Time,
	DateTime {
		precision: u8,
		timezone: String,
	},
	Array(Box<DataType>),
	Map {
		key: Box<DataType>,
		value: Box<DataType>,
	},
	Nullable(Box<DataType>),
	LowCardinality(Box<DataType>),
	Tuple(Vec<(String, DataType)>),
	Json,
	#[default]
	Unknown,
}

impl DataType {
	pub fn parse(type_str: String, db_type: DbType) -> IResult<DataType> {
		match db_type {
			DbType::MySQL => type_parser_mysql::parse(type_str),
			DbType::ClickHouse => type_parser_ch::parse(type_str),
			DbType::HBase => type_parser_hbase::parse(type_str),
			_ => Ok(Default::default()),
		}
	}
}
