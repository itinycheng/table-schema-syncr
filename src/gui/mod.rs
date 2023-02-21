use iced::{
	widget::{self, Column, Row},
	Application, Command, Element, Subscription,
};

use crate::db;

mod content;
mod header;
mod modal;
mod sidebar;
mod style;

#[derive(Debug, Default)]
pub struct App {
	pub show_conn_modal: bool,
	pub connection_params: ConnectionParams,
}

#[derive(Debug, Default)]
pub struct ConnectionParams {
	pub url: String,
	pub username: String,
	pub password: String,
	pub db_type: String,
}

#[derive(Debug, Clone)]
pub enum Message {
	OpenConnectionForm,
	SubmitConnectionForm,
	CloseConnectionForm,
	ConnectionUrl(String),
	ConnectionUsername(String),
	ConnectionPassword(String),
	ConnectionDbType(String),
	Event(iced::Event),
	Nothing,
}

impl Application for App {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = iced::theme::Theme;
	type Flags = ();

	fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
		(App::default(), Command::none())
	}

	fn title(&self) -> String {
		String::from("Table Syncr - Iced")
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		iced::subscription::events().map(Message::Event)
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::OpenConnectionForm => {
				self.show_conn_modal = true;
				widget::focus_next()
			}
			Message::CloseConnectionForm => {
				self.show_conn_modal = false;
				self.connection_params = ConnectionParams::default();
				widget::focus_next()
			}
			Message::SubmitConnectionForm => {
				self.show_conn_modal = false;
				db::save_conn_params(&self.connection_params);
				self.connection_params = ConnectionParams::default();
				widget::focus_next()
			}
			Message::ConnectionDbType(db_type) => {
				self.connection_params.db_type = db_type;
				Command::none()
			}
			Message::ConnectionUrl(url) => {
				self.connection_params.url = url;
				Command::none()
			}
			Message::ConnectionUsername(username) => {
				self.connection_params.username = username;
				Command::none()
			}
			Message::ConnectionPassword(password) => {
				self.connection_params.password = password;
				Command::none()
			}
			Message::Nothing => Command::none(),
			_ => Command::none(),
		}
	}

	fn view(&self) -> Element<Message> {
		Column::new()
			.push(header::view(self))
			.push(Row::new().push(sidebar::view()).push(content::view(self)))
			.padding(10)
			.into()
	}
}
