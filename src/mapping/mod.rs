pub mod column;
pub mod database;
pub mod table;
mod type_parser_ch;
mod type_parser_hbase;
mod type_parser_mysql;

fn is_balanced_brackets(s: &str) -> bool {
	let mut stack = Vec::new();
	for c in s.chars() {
		match c {
			'(' => stack.push(c),
			')' => {
				if stack.pop().is_none() {
					return false;
				}
			}
			_ => (),
		}
	}
	stack.is_empty()
}

trait FindSkip {
	fn find_skip(&self, p: char, num: usize) -> Option<usize>;
}

impl FindSkip for str {
	fn find_skip(&self, p: char, num: usize) -> Option<usize> {
		self.chars().enumerate().filter(|&(_, c)| c == p).nth(num).map(|(idx, _)| idx)
	}
}
