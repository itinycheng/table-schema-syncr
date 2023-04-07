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
	Int {
		size: usize,
		unsigned: bool,
	},
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

	pub fn to_type(&self, db_type: DbType) -> String {
		match db_type {
			DbType::MySQL => self.to_mysql_type(),
			DbType::ClickHouse => self.to_ch_type(),
			DbType::HBase => "".to_owned(),
			DbType::Internal => format!("{:?}", self),
			DbType::Unknown => panic!("Unsupported db type"),
		}
	}

	fn to_mysql_type(&self) -> String {
		match self {
			DataType::Int { size, unsigned } => {
				const INT_TYPES: [&str; 6] =
					["NULL", "TINYINT", "SMALLINT", "MEDIUMINT", "INT", "BIGINT"];
				let type_string = INT_TYPES.get(*size).unwrap_or(&INT_TYPES[5]).to_string();
				type_string + (if *unsigned { " unsigned" } else { "" })
			}
			DataType::Float(size) => match size {
				&4 => "FLOAT",
				_ => "DOUBLE",
			}
			.to_owned(),
			DataType::Decimal { precision, scale } => {
				format!("DECIMAL({}, {})", precision, scale)
			}
			DataType::Bool => "BOOL".to_owned(),
			DataType::String(size_opt) => match size_opt {
				&Some(size) if size > 65535 => "LONGTEXT".to_owned(),
				&Some(size) => format!("VARCHAR({})", size),
				None => "VARCHAR(255)".to_owned(),
			},
			DataType::Uuid => "VARCHAR(36)".to_owned(),
			DataType::Date => "DATE".to_owned(),
			DataType::Time => "TIME".to_owned(),
			DataType::DateTime { .. } => "DATETIME".to_owned(),
			DataType::Tuple(_) | DataType::Array(_) | DataType::Map { .. } => "VARCHAR".to_owned(),
			DataType::Nullable(data_type) | DataType::LowCardinality(data_type) => {
				data_type.to_mysql_type()
			}
			DataType::Json => "JSON".to_owned(),
			DataType::Unknown => "Unknown".to_owned(),
		}
	}

	fn to_ch_type(&self) -> String {
		match self {
			DataType::Int { size, unsigned } => {
				format!("{}{}", if *unsigned { "Uint" } else { "Int" }, size * 8)
			}
			DataType::Float(size) => {
				format!("Float{}", size * 8)
			}
			DataType::Decimal { precision, scale } => {
				format!("Decimal({},{})", precision, scale)
			}
			DataType::Bool => "Bool".to_owned(),
			DataType::String(_) => "String".to_owned(),
			DataType::Uuid => "UUID".to_owned(),
			DataType::Date => "Date".to_owned(),
			DataType::Time => "DateTime()".to_owned(),
			DataType::DateTime { precision, timezone } => {
				let mut buf = precision.to_string();
				if let Some(tz) = timezone {
					buf.push(',');
					buf.push_str(tz);
				}
				format!("Decimal64({})", buf)
			}
			DataType::Array(sub_type) => {
				format!("Array({})", Self::to_ch_type(sub_type))
			}
			DataType::Map { key, value } => {
				format!("Map({}, {})", Self::to_ch_type(key), Self::to_ch_type(value))
			}
			DataType::Nullable(sub_type) => {
				format!("Nullable({})", Self::to_ch_type(sub_type))
			}
			DataType::LowCardinality(sub_type) => {
				format!("LowCardinality({})", Self::to_ch_type(sub_type))
			}
			DataType::Tuple(sub_types) => {
				let joined = sub_types
					.into_iter()
					.map(|tuple| {
						if tuple.0.len() > 0 {
							format!("{} {}", &tuple.0, Self::to_ch_type(&tuple.1))
						} else {
							tuple.0.to_owned()
						}
					})
					.collect::<Vec<_>>()
					.join(",");
				format!("Tuple({})", joined)
			}
			DataType::Json => "JSON".to_owned(),
			DataType::Unknown => "Unknown".to_owned(),
		}
	}
}
