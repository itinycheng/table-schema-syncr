use iced::{
	widget::{self, Column, Row},
	Application, Command, Element, Length, Subscription,
};

use crate::{
	conn::DBClient,
	database::DbType,
	error::{IError, IResult},
	store::conn_conf::{self, ConnConf},
};

use self::toast::Toast;

mod content;
mod event_handler;
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
	pub selected_conn: Option<String>,
	pub databases: Vec<String>,
	pub selected_db: Option<String>,
	pub tables: Vec<String>,
	pub selected_table: Option<String>,
	pub toasts: Vec<Toast>,
}

#[derive(Debug, Clone)]
pub enum Message {
	EditConnection(Option<usize>),
	DeleteConnection(usize),
	SelectedConnection(String),
	ShowDatabases(Option<Vec<String>>),
	SelectedDatabase(String),
	ShowTables(Option<Vec<String>>),
	SelectedTable(String),
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
		iced::subscription::events().map(Message::IcedEvent)
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
			Message::SelectedConnection(uuid) => {
				self.selected_conn = Some(uuid.clone());
				self.selected_db = None;
				self.selected_table = None;
				self.tables = vec![];
				self.databases = vec![];
				Command::perform(
					async move {
						let conf = conn_conf::query_by_uuid(&uuid).ok()?;
						let db_client = DBClient::get_or_init(conf.try_into().ok()?).ok()?;
						let databases = db_client.databases().ok()?;
						Some(databases)
					},
					Message::ShowDatabases,
				)
			}
			Message::ShowDatabases(databases) => {
				if let Some(databases) = databases {
					self.databases = databases;
				} else {
					self.databases = vec![]
				}
				Command::none()
			}
			Message::SelectedDatabase(database) => {
				self.selected_db = Some(database.clone());
				self.selected_table = None;
				self.tables = vec![];
				let uuid = self.selected_conn.clone().unwrap();
				Command::perform(
					async move {
						let conf = conn_conf::query_by_uuid(&uuid).ok()?;
						let db_client = DBClient::get_or_init(conf.try_into().ok()?).ok()?;
						let tables = db_client.tables(&database).ok()?;
						Some(tables)
					},
					Message::ShowTables,
				)
			}
			Message::ShowTables(tables) => {
				if let Some(tables) = tables {
					self.tables = tables;
				} else {
					self.tables = vec![]
				}
				Command::none()
			}
			Message::SelectedTable(table) => {
				self.selected_table = Some(table);
				Command::none()
			}
			Message::CloseConnForm => {
				self.show_conn_modal = false;
				self.edit_conn = ConnConf::default();
				widget::focus_next()
			}
			Message::SubmitConnForm => {
				fn modify_and_fetch_all(edit_conn: &ConnConf) -> IResult<Vec<ConnConf>> {
					conn_conf::insert_or_update(edit_conn)?;
					conn_conf::list_all()
				}

				match modify_and_fetch_all(&self.edit_conn) {
					Ok(conns) => {
						self.all_conns = conns;
						self.show_conn_modal = false;
						self.edit_conn = ConnConf::default();
						widget::focus_next()
					}
					Err(e) => {
						self.display_err(&e);
						Command::none()
					}
				}
			}
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
			Message::IcedEvent(e) => event_handler::handle(e),
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

impl App {
	pub fn display_err(&mut self, e: &IError) {
		self.toasts.push(Toast {
			title: "Error".into(),
			body: e.to_string(),
			status: toast::Status::Danger,
		});
	}
}
