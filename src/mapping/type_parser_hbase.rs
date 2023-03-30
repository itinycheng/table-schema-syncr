use crate::error::IResult;

use super::column::DataType;

pub fn parse<T: AsRef<str>>(_: T) -> IResult<DataType> {
	Ok(DataType::Date)
}
