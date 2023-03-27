use crate::error::IResult;

use super::column::DataType;

pub fn parse(_: String) -> IResult<DataType> {
	Ok(DataType::Date)
}
