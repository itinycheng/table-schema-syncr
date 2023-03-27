use iced::{
	widget::{self, Column, Row},
	Application, Command, Element, Length, Subscription,
};

use crate::{
	conn::DBClient,
	error::{IError, IResult},
	mapping::{column::ColumnSpec, database::DbType},
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
	pub selected_db_type: Option<DbType>,
	pub origin_table_schema: Vec<ColumnSpec>,
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
	SelectedDBType(DbType),
	ShowTableSchema(Option<Vec<ColumnSpec>>),
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
		String::from("Table Sync - Iced")
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
				self.reset_connection(&uuid);
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
				self.databases = databases.unwrap_or_default();
				Command::none()
			}
			Message::SelectedDatabase(database) => {
				self.reset_database(&database);
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
				self.tables = tables.unwrap_or_default();
				Command::none()
			}
			Message::SelectedTable(table) => {
				self.selected_table.replace(table);
				self.selected_db_type.take();
				Command::none()
			}
			Message::SelectedDBType(db_type) => {
				self.selected_db_type.replace(db_type);
				let conn_uuid = self.selected_conn.clone().unwrap();
				let database = self.selected_db.clone().unwrap();
				let table = self.selected_table.clone().unwrap();
				Command::perform(
					async move {
						let conf = conn_conf::query_by_uuid(&conn_uuid).ok()?;
						let db_client = DBClient::get_or_init(conf.try_into().ok()?).ok()?;
						Some(db_client.table_schema(&database, &table).ok()?)
					},
					Message::ShowTableSchema,
				)
			}
			Message::ShowTableSchema(schema) => {
				self.origin_table_schema = schema.unwrap_or_default();
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
				self.edit_conn.db_type.replace(db_type);
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
	pub fn reset_connection(&mut self, conn_uuid: &String) {
		self.selected_conn.replace(conn_uuid.clone());
		self.selected_db.take();
		self.selected_table.take();
		self.selected_db_type.take();
		self.tables.clear();
		self.databases.clear();
		self.origin_table_schema.clear();
	}

	pub fn reset_database(&mut self, database: &String) {
		self.selected_db.replace(database.clone());
		self.selected_table.take();
		self.selected_db_type.take();
		self.tables.clear();
		self.origin_table_schema.clear();
	}

	pub fn display_err(&mut self, e: &IError) {
		self.toasts.push(Toast {
			title: "Error".into(),
			body: e.to_string(),
			status: toast::Status::Danger,
		});
	}
}
