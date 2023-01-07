pub struct Column {
	pub name: String,
	pub r#type: DataType,
	pub nullable: bool,
	pub comment: String,
}

pub enum DataType {
	Int(usize),
	Float(usize),
	Decimal { precision: u8, scale: u8 },
	String(usize),
	Date,
	Time,
	DateTime { precision: u8, timezone: String },
	Array(Box<DataType>),
	Map { key: Box<DataType>, value: Box<DataType> },
}
