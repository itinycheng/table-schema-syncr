use crate::error::{IError, IResult};

use super::{column::DataType, is_balanced_brackets, FindSkip};

pub fn parse<T: AsRef<str>>(type_str: T) -> IResult<DataType> {
	Ok(match type_str.as_ref().trim() {
		"Int8" => DataType::Int(8),
		"Int16" => DataType::Int(16),
		"Int32" => DataType::Int(32),
		"Int64" => DataType::Int(64),
		"Int128" => DataType::Int(128),
		"Int256" => DataType::Int(256),
		"UInt8" => DataType::UInt(8),
		"UInt16" => DataType::UInt(16),
		"UInt32" => DataType::UInt(32),
		"UInt64" => DataType::UInt(64),
		"UInt128" => DataType::UInt(128),
		"UInt256" => DataType::UInt(256),
		"Float32" => DataType::Float(32),
		"Float64" => DataType::Float(64),
		"Bool" => DataType::Bool,
		"IPv4" => DataType::UInt(32),
		"IPv6" => DataType::String(Some(16)),
		"String" => DataType::String(None),
		s if s.starts_with("FixedString") => DataType::String(Some(s[12..(s.len() - 1)].parse()?)),
		"UUID" => DataType::Uuid,
		"Date" | "Date32" => DataType::Date,
		s if s.starts_with("Decimal(") => {
			let sub_str = &s[8..(s.len() - 1)];
			let idx = sub_str.find(",").expect("Wrong decimal type!");
			DataType::Decimal {
				precision: sub_str[..idx].parse()?,
				scale: sub_str[(idx + 1)..].parse()?,
			}
		}
		s if s.starts_with("Decimal32") => {
			let scale: u8 = s[10..(s.len() - 1)].parse()?;
			let precision = 10 - (f64::log10(scale as f64)).ceil() as u8;
			DataType::Decimal { precision, scale }
		}
		s if s.starts_with("Decimal64") => {
			let scale: u8 = s[10..(s.len() - 1)].parse()?;
			let precision = 19 - (f64::log10(scale as f64)).ceil() as u8;
			DataType::Decimal { precision, scale }
		}
		s if s.starts_with("Decimal128") => {
			let scale: u8 = s[11..(s.len() - 1)].parse()?;
			let precision = 39 - (f64::log10(scale as f64)).ceil() as u8;
			DataType::Decimal { precision, scale }
		}
		s if s.starts_with("Decimal256") => {
			let scale: u8 = s[11..(s.len() - 1)].parse()?;
			let precision = 77 - (f64::log10(scale as f64)).ceil() as u8;
			DataType::Decimal { precision, scale }
		}
		s if s.starts_with("DateTime(") || s == "DateTime" => DataType::DateTime {
			precision: 0,
			timezone: if s.len() > 9 { Some(s[9..(s.len() - 1)].replace('\'', "")) } else { None },
		},
		s if s.starts_with("DateTime64") => {
			let mut parts = s[11..(s.len() - 1)].splitn(2, ',');
			let precision = parts.next().expect("Wrong DateTime64 type").parse::<u8>()?;
			let timezone =
				if let Some(zone) = parts.next() { Some(zone.replace('\'', "")) } else { None };
			DataType::DateTime { precision, timezone }
		}
		s if s.starts_with("Nullable") => {
			let sub_str = &s[9..(s.len() - 1)];
			DataType::Nullable(Box::new(parse(sub_str)?))
		}
		s if s.starts_with("Array") => {
			let sub_str = &s[6..(s.len() - 1)];
			DataType::Array(Box::new(parse(sub_str)?))
		}
		s if s.starts_with("LowCardinality") => {
			let sub_str = &s[15..(s.len() - 1)];
			DataType::LowCardinality(Box::new(parse(sub_str)?))
		}
		s if s.starts_with("Map") => {
			let sub_str = &s[4..(s.len() - 1)];
			let idx = sub_str.find(',').expect("Wrong map type!");
			DataType::Map {
				key: Box::new(parse(&sub_str[..idx])?),
				value: Box::new(parse(&sub_str[(idx + 1)..])?),
			}
		}
		s if s.starts_with("Tuple") => {
			let mut name_type_tuples = Vec::new();
			let mut remaining = &s[6..(s.len() - 1)];

			let mut skip = 0;
			while !remaining.is_empty() {
				if let Some(idx) = remaining.find_skip(',', skip) {
					let name_type_str = &remaining[..idx];
					if is_balanced_brackets(name_type_str) {
						name_type_tuples.push(parse_sub_type(name_type_str)?);
						remaining = &remaining[(idx + 1)..];
						skip = 0;
					} else {
						skip += 1;
					}
				} else {
					name_type_tuples.push(parse_sub_type(remaining)?);
					remaining = "";
				}
			}
			DataType::Tuple(name_type_tuples)
		}
		"JSON" | "Json" => DataType::Json,
		s if s.starts_with("Enum")
			|| s.starts_with("AggregateFunction")
			|| s.starts_with("SimpleAggregateFunction") =>
		{
			Err(IError::PromptError("Unimplemented type: ".to_owned()))?
		}
		_ => DataType::Unknown,
	})
}

fn parse_sub_type(name_type_str: &str) -> IResult<(String, DataType)> {
	let name_type_vec = name_type_str.trim().splitn(2, ' ').collect::<Vec<_>>();
	Ok(match name_type_vec.len() {
		1 => ("".to_string(), parse(name_type_vec.get(0).unwrap())?),
		2 => (name_type_vec.get(0).unwrap().to_string(), parse(name_type_vec.get(1).unwrap())?),
		_ => unreachable!(),
	})
}

#[cfg(test)]
mod tests {

	use super::parse;

	#[test]
	fn test_parse() {
		println!("{:?}", parse("Map(String, Int32)"));
		println!("{:?}", parse("Array(Nullable(Int8))"));
		println!("{:?}", parse("Map(LowCardinality(String), Int32)"));
		println!("{:?}", parse("Map(String, Array(Decimal64(18)))"));
		println!(
			"{:?}",
			parse("Tuple(Array(String), s Map(String, Int64) , s Map(String, Int64))")
		);
	}
}
