use mysql::consts::ColumnType;

use crate::error::{IError, IResult};

use super::column::DataType;

pub fn parse(column_type: ColumnType) -> IResult<DataType> {
	Ok(match column_type {
		ColumnType::MYSQL_TYPE_DECIMAL | ColumnType::MYSQL_TYPE_NEWDECIMAL => {
			DataType::Decimal { precision: 10, scale: 0 }
		}
		ColumnType::MYSQL_TYPE_TINY => DataType::Int(8),
		ColumnType::MYSQL_TYPE_SHORT => DataType::Int(16),
		ColumnType::MYSQL_TYPE_LONG => DataType::Int(64),
		ColumnType::MYSQL_TYPE_FLOAT => DataType::Float(32),
		ColumnType::MYSQL_TYPE_DOUBLE => DataType::Float(64),
		ColumnType::MYSQL_TYPE_NULL => DataType::Null,
		ColumnType::MYSQL_TYPE_TIMESTAMP | ColumnType::MYSQL_TYPE_TIMESTAMP2 => {
			DataType::DateTime { precision: 0, timezone: None }
		}
		ColumnType::MYSQL_TYPE_LONGLONG => DataType::Int(128),
		ColumnType::MYSQL_TYPE_INT24 => DataType::Int(24),
		ColumnType::MYSQL_TYPE_DATE | ColumnType::MYSQL_TYPE_NEWDATE => DataType::Date,
		ColumnType::MYSQL_TYPE_TIME | ColumnType::MYSQL_TYPE_TIME2 => DataType::Time,
		ColumnType::MYSQL_TYPE_DATETIME | ColumnType::MYSQL_TYPE_DATETIME2 => {
			DataType::DateTime { precision: 3, timezone: None }
		}
		ColumnType::MYSQL_TYPE_YEAR => todo!(),
		ColumnType::MYSQL_TYPE_VARCHAR
		| ColumnType::MYSQL_TYPE_STRING
		| ColumnType::MYSQL_TYPE_VAR_STRING => DataType::String(None),
		ColumnType::MYSQL_TYPE_BIT => DataType::Bool,
		ColumnType::MYSQL_TYPE_TYPED_ARRAY => DataType::Array(Box::new(DataType::String(None))),
		ColumnType::MYSQL_TYPE_UNKNOWN => DataType::Unknown,
		ColumnType::MYSQL_TYPE_JSON => DataType::Json,
		ColumnType::MYSQL_TYPE_TINY_BLOB
		| ColumnType::MYSQL_TYPE_MEDIUM_BLOB
		| ColumnType::MYSQL_TYPE_BLOB
		| ColumnType::MYSQL_TYPE_LONG_BLOB => DataType::String(None),
		ColumnType::MYSQL_TYPE_GEOMETRY
		| ColumnType::MYSQL_TYPE_ENUM
		| ColumnType::MYSQL_TYPE_SET => Err(IError::PromptError("Unsupported type".to_owned()))?,
	})
}
