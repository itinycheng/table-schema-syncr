pub fn add(left: usize, right: usize) -> usize {
	left + right
}

mod clickhouse;
mod column;
mod table;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
