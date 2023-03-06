use iced::{
	widget::{self, Column, Row},
	Application, Command, Element, Length, Subscription,
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
	pub edit_conn: ConnConf,
	pub all_conns: Vec<ConnConf>,
	pub toasts: Vec<Toast>,
}

#[derive(Debug, Clone)]
pub enum Message {
	EditConnection(Option<usize>),
	DeleteConnection(usize),
	SubmitConnForm,
	CloseConnForm,
	EditConnName(String),
	EditConnDbType(DbType),
	EditConnUrl(String),
	EditConnUsername(String),
	EditConnPassword(String),
	IcedEvent(iced::Event),
	CloseToast(usize),
	Nothing,
}

impl Application for App {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = iced::theme::Theme;
	type Flags = ();

	fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
		(App { all_conns: conn_conf::list_all().unwrap(), ..Default::default() }, Command::none())
	}

	fn title(&self) -> String {
		String::from("Table Syncr - Iced")
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		// iced::subscription::events().map(Message::IcedEvent)
		Subscription::none()
	}

	fn update(&mut self, message: Message) -> Command<Message> {
		match message {
			Message::EditConnection(idx_opt) => {
				if let Some(idx) = idx_opt {
					self.all_conns
						.get(idx)
						.into_iter()
						.for_each(|cached| self.edit_conn = cached.clone())
				}

				self.show_conn_modal = true;
				widget::focus_next()
			}
			Message::DeleteConnection(idx) => {
				if self
					.all_conns
					.get(idx)
					.map(|cached| conn_conf::delete(&cached.uuid).is_ok())
					.is_some()
				{
					self.all_conns.remove(idx);
				}

				Command::none()
			}
			Message::CloseConnForm => {
				self.show_conn_modal = false;
				self.edit_conn = ConnConf::default();
				widget::focus_next()
			}
			Message::SubmitConnForm => match conn_conf::insert_or_update(&self.edit_conn) {
				Ok(_) => {
					self.show_conn_modal = false;
					self.edit_conn = ConnConf::default();
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
				self.edit_conn.name = name;
				Command::none()
			}
			Message::EditConnDbType(db_type) => {
				self.edit_conn.db_type = Some(db_type);
				Command::none()
			}
			Message::EditConnUrl(url) => {
				self.edit_conn.url = url;
				Command::none()
			}
			Message::EditConnUsername(username) => {
				self.edit_conn.username = username;
				Command::none()
			}
			Message::EditConnPassword(password) => {
				self.edit_conn.password = password;
				Command::none()
			}
			Message::CloseToast(index) => {
				self.toasts.remove(index);
				Command::none()
			}
			_ => Command::none(),
		}
	}

	fn view(&self) -> Element<Message> {
		let user_view = Column::new()
			.push(header::view(self))
			.push(Row::new().push(sidebar::view(self)).push(content::view(self)))
			.padding(10)
			.width(Length::Fill);

		toast::Manager::new(user_view, &self.toasts, Message::CloseToast).timeout(2).into()
	}
}
