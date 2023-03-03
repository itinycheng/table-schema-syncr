use iced::{
	widget::{self, Column, Row},
	Application, Command, Element, Subscription, Length,
};

use crate::{
	database::DbType,
	store::conn_conf::{self, ConnConf},
};

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

#[derive(Debug, Clone)]
pub enum Message {
	OpenConnectionForm,
	SubmitConnectionForm,
	CloseConnectionForm,
	EditConnName(String),
	EditConnDbType(DbType),
	EditConnUrl(String),
	EditConnUsername(String),
	EditConnPassword(String),
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
			Message::SubmitConnectionForm => match conn_conf::insert(&self.conn_conf) {
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
			Message::EditConnName(name) => {
				self.conn_conf.name = name;
				Command::none()
			}
			Message::EditConnDbType(db_type) => {
				self.conn_conf.db_type = Some(db_type);
				Command::none()
			}
			Message::EditConnUrl(url) => {
				self.conn_conf.url = url;
				Command::none()
			}
			Message::EditConnUsername(username) => {
				self.conn_conf.username = username;
				Command::none()
			}
			Message::EditConnPassword(password) => {
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
			.padding(10)
			.width(Length::Fill);

		toast::Manager::new(user_view, &self.toasts, Message::CloseToast).timeout(2).into()
	}
}
