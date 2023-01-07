use crate::database::DbType;

use super::column::Column;

pub struct Table<'a> {
	pub name: String,
	pub database: String,
	pub r#type: DbType,
	pub columns: Vec<Column>,
	pub primary_keys: Option<Vec<&'a Column>>,
	pub order_by: Option<Vec<&'a Column>>,
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

		let mut self_ref = SelfRef { value: vec!["value".to_string()], reference: None };
		self_ref.reference = Some(vec![self_ref.value.get(0).unwrap()]);

		println!(
			"{:p}, {:p}",
			self_ref.value.get(0).unwrap(),
			*self_ref.reference.unwrap().get(0).unwrap()
		)
	}
}
