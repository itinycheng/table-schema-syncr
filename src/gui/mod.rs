use iced::{
	widget::{self, Column, Row},
	Application, Command, Element, Subscription,
};
use validator::Validate;

use crate::store;

use self::toast::Toast;

mod content;
mod header;
mod modal;
mod sidebar;
mod style;
mod toast;

#[derive(Debug, Default)]
pub struct App {
	pub show_conn_modal: bool,
	pub conn_conf: ConnConf,
	pub toasts: Vec<Toast>,
}

#[derive(Debug, Default, Validate)]
pub struct ConnConf {
	#[validate(length(min = 1))]
	pub db_type: String,
	#[validate(length(min = 1))]
	pub url: String,
	#[validate(length(min = 1))]
	pub username: String,
	pub password: String,
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
	CloseToast(usize),
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
				self.conn_conf = ConnConf::default();
				widget::focus_next()
			}
			Message::SubmitConnectionForm => match store::save_conn_conf(&self.conn_conf) {
				Ok(_) => {
					self.show_conn_modal = false;
					self.conn_conf = ConnConf::default();
					widget::focus_next()
				}
				Err(e) => {
					self.toasts.push(Toast {
						title: "Error".into(),
						body: e.to_string(),
						status: toast::Status::Danger,
					});
					Command::none()
				}
			},
			Message::ConnectionDbType(db_type) => {
				self.conn_conf.db_type = db_type;
				Command::none()
			}
			Message::ConnectionUrl(url) => {
				self.conn_conf.url = url;
				Command::none()
			}
			Message::ConnectionUsername(username) => {
				self.conn_conf.username = username;
				Command::none()
			}
			Message::ConnectionPassword(password) => {
				self.conn_conf.password = password;
				Command::none()
			}
			Message::CloseToast(index) => {
				self.toasts.remove(index);
				Command::none()
			}
			Message::Nothing => Command::none(),
			_ => Command::none(),
		}
	}

	fn view(&self) -> Element<Message> {
		let user_view = Column::new()
			.push(header::view(self))
			.push(Row::new().push(sidebar::view()).push(content::view(self)))
			.padding(10);

		toast::Manager::new(user_view, &self.toasts, Message::CloseToast).timeout(2).into()
	}
}
