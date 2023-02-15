use gui::MyStyle;
use iced::{
	theme,
	widget::{button, container, row},
	Element, Length, Sandbox, Settings,
};

use log::info;

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
	Counter::run(Settings::default())?;
	Ok(())
}

#[derive(Debug, Default)]
struct Counter {
	value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
	StoreDbProperties,
	DecrementPressed,
}

impl Sandbox for Counter {
	type Message = Message;

	fn new() -> Self {
		Counter::default()
	}

	fn title(&self) -> String {
		String::from("Table Syncr - Iced")
	}

	fn update(&mut self, message: Message) {
		match message {
			Message::StoreDbProperties => {
				self.value += 1;
				info!("press increment");
			}
			Message::DecrementPressed => {
				self.value -= 1;
				info!("press decrement");
			}
		}
	}

	fn view(&self) -> Element<Message> {
		container(
			row![
				button("Create").on_press(Message::StoreDbProperties),
				button("Decrement").on_press(Message::DecrementPressed)
			]
			.spacing(20),
		)
		.width(Length::Fill)
		.style(theme::Container::Custom(Box::new(MyStyle)))
		.padding(10)
		.into()
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn it_works() {
		let a: i32 = 1.into();
		let b: i32 = From::from(2);
		let c: Box<i32> = 3.into();
		println!("{} {} {}", a, b, c)
	}
}
