use super::database::DbType;

pub struct Column {
	pub name: String,
	pub r#type: DataType,
	pub nullable: bool,
	pub comment: String,
}

impl Column {

}

#[derive(Default)]
pub enum DataType {
	Int(usize),
	Float(usize),
	Decimal {
		precision: u8,
		scale: u8,
	},
	String(usize),
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
	#[default]
	Unknown,
}

impl DataType {

	pub fn parse(type_str: String, db_type: DbType) -> DataType {
		match db_type {
			DbType::MySQL => Self::parse_mysql_type(type_str),
			DbType::ClickHouse => Self::parse_ch_type(type_str),
			DbType::HBase => Self::parse_hbase_type(type_str),
			_ => Default::default(),
		}
	}

	fn parse_ch_type(type_str: String) -> DataType {
		DataType::Date
	}

	fn parse_mysql_type(type_str: String) -> DataType {
		DataType::Date
	}

	fn parse_hbase_type(type_str: String) -> DataType {
		DataType::Date
	}
}
