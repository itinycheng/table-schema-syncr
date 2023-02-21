use gui::App;
use iced::{Application, Settings};

pub mod column;
pub mod conf;
pub mod conn;
pub mod database;
pub mod db;
pub mod error;
pub mod gui;
pub mod table;
pub mod util;

pub fn main() -> error::IResult<()> {
	conf::app_init();
	App::run(Settings::default())?;
	Ok(())
}

#[cfg(test)]
mod tests {

	trait Content {
		fn test(&self);
	}

	impl<T> Content for T
	where
		T: Fn(String) -> i32,
	{
		fn test(&self) {
			println!("fn(&String) -> i32");
		}
	}

	pub fn age(str: String) -> i32 {
		str.parse::<i32>().unwrap()
	}

	#[test]
	fn it_works() {
		let a: i32 = 1.into();
		let b: i32 = From::from(2);
		let c: Box<i32> = 3.into();
		println!("{} {} {}", a, b, c);

		let func = age;
		func.test();
	}
}
