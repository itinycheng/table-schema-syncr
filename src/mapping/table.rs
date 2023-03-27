use super::{column::ColumnSpec, database::DbType};

pub struct Table<'a> {
	pub name: String,
	pub database: String,
	pub r#type: DbType,
	pub columns: Vec<ColumnSpec>,
	pub primary_keys: Option<Vec<&'a ColumnSpec>>,
	pub order_by: Option<Vec<&'a ColumnSpec>>,
	pub engine: String,
}

#[cfg(test)]
mod tests {

	#[test]
	fn test_ref_to_self_location() {
		#[derive(Debug)]
		struct SelfRef<'a> {
			value: Vec<String>,
			reference: Option<Vec<&'a String>>,
		}

		let mut self_ref =
			SelfRef { value: vec!["value0".to_string(), "value1".to_string()], reference: None };
		self_ref.reference = Some(vec![self_ref.value.get(0).unwrap()]);
		self_ref.value.remove(0);

		// println!(
		// 	"{:p}, {:p}",
		// 	self_ref.value.get(0).unwrap(),
		// 	*self_ref.reference.unwrap().get(0).unwrap()
		// );
		println!("end");
	}
}
