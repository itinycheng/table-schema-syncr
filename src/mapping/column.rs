use core::any::Any;
use mysql::consts::ColumnType;

use crate::error::{IError, IResult};

use super::{database::DbType, type_parser_ch, type_parser_hbase, type_parser_mysql};

#[derive(Debug, Default, Clone)]
pub struct ColumnSpec {
	pub name: String,
	pub r#type: DataType,
	pub comment: String,
}

impl ColumnSpec {
	pub fn create<T: Any + 'static>(
		name: String,
		any_type: T,
		comment: String,
		db_type: DbType,
	) -> IResult<ColumnSpec> {
		Ok(ColumnSpec { name: name, r#type: DataType::parse(any_type, db_type)?, comment })
	}
}

#[derive(Debug, Default, Clone)]
pub enum DataType {
	Null,
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
		timezone: Option<String>,
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
	pub fn parse<T: Any + 'static>(any_type: T, db_type: DbType) -> IResult<DataType> {
		match db_type {
			DbType::MySQL => type_parser_mysql::parse(
				*(&any_type as &dyn Any).downcast_ref::<ColumnType>().ok_or(
					IError::PromptError("Can't downcast type value to ColumnType".to_owned()),
				)?,
			),
			DbType::ClickHouse => type_parser_ch::parse(
				&(&any_type as &dyn Any)
					.downcast_ref::<String>()
					.ok_or(IError::PromptError("Can't downcast type value to String".to_owned()))?[..],
			),
			DbType::HBase => type_parser_hbase::parse(
				&(&any_type as &dyn Any)
					.downcast_ref::<String>()
					.ok_or(IError::PromptError("Can't downcast type value to String".to_owned()))?[..],
			),
			_ => Ok(Default::default()),
		}
	}
}
