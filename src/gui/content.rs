use iced::{
	alignment::Vertical,
	theme,
	widget::{
		button, column, container, pick_list, row, scrollable, text, text_input, Column, Container,
		Row,
	},
	Length, Renderer,
};

use super::{
	style::{border_style, button_style},
	App, Message,
};
use crate::{database::DbType, gui::modal::Modal};

pub fn view(app: &App) -> Container<Message, Renderer> {
	let databases = scrollable(show_databases(app))
		.horizontal_scroll(scrollable::Properties::new().width(1.0).scroller_width(2.0));
	let tables = scrollable(show_tables(app))
		.horizontal_scroll(scrollable::Properties::new().width(1.0).scroller_width(2.0));
	let content = Column::new()
		.push(databases)
		.push(tables)
		.push(show_db_types(app))
		.push(show_table_schema(app));

	let mut content_wrapper = container(scrollable(content))
		.width(Length::FillPortion(4))
		.height(Length::Fill)
		.padding(5)
		.style(border_style());

	if app.show_conn_modal {
		content_wrapper = container(
			Modal::new(content_wrapper, edit_conn_form(app)).on_blur(Message::CloseConnForm),
		)
		.width(Length::FillPortion(4))
		.height(Length::Fill)
		.style(border_style())
	}

	content_wrapper
}

fn show_table_schema(app: &App) -> Row<'_, Message, Renderer> {
	if app.origin_table_schema.is_none() {
		Row::new()
	} else {
		Row::new().push(text(app.origin_table_schema.as_ref().unwrap()))
	}
	.width(Length::Fill)
	.padding(5)
	.spacing(5)
}

fn show_db_types(app: &App) -> Row<'_, Message, Renderer> {
	if app.selected_table.is_none() {
		Row::new()
	} else {
		DbType::ALL.iter().fold(Row::new(), |base, db_type| {
			base.push(
				button(text(db_type))
					.height(30.0)
					.style(button_style(
						matches!(&app.selected_db_type, Some(selected) if selected == db_type),
					))
					.on_press(Message::SelectedDBType(db_type.clone())),
			)
		})
	}
	.height(40.0)
	.padding(5)
	.spacing(5)
}

fn show_tables(app: &App) -> Row<'_, Message, Renderer> {
	if app.tables.is_empty() {
		Row::new()
	} else {
		app.tables.iter().fold(Row::new(), |base, table| {
			base.push(
				button(text(table))
					.height(30.0)
					.style(button_style(
						matches!(&app.selected_table, Some(selected) if selected == table),
					))
					.on_press(Message::SelectedTable(table.clone())),
			)
		})
	}
	.height(Length::Fixed(40.0))
	.padding(5)
	.spacing(5)
}

fn show_databases(app: &App) -> Row<'_, Message, Renderer> {
	if app.databases.is_empty() {
		Row::new().push("nothing")
	} else {
		app.databases.iter().fold(Row::new(), |base, database| {
			base.push(
				button(text(database))
					.height(30.0)
					.style(button_style(
						matches!(&app.selected_db, Some(selected) if selected == database),
					))
					.on_press(Message::SelectedDatabase(database.clone())),
			)
		})
	}
	.height(Length::Fixed(40.0))
	.padding(5)
	.spacing(5)
}

fn edit_conn_form<'a>(app: &App) -> Container<'a, Message, Renderer> {
	container(
		column![
			text("New Connection").size(20),
			row![
				text("Name")
					.size(16)
					.width(Length::Fixed(80.0))
					.vertical_alignment(Vertical::Center),
				text_input("name", &app.edit_conn.name, Message::EditConnName)
			]
			.spacing(5),
			column![
				row![
					text("Type")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					pick_list(&DbType::ALL[..], app.edit_conn.db_type, Message::EditConnDbType,)
						.width(Length::Fill)
						.placeholder("Choose database type...")
				]
				.spacing(5),
				row![
					text("Url")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("url", &app.edit_conn.url, Message::EditConnUrl)
				]
				.spacing(5),
				row![
					text("Username")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("username", &app.edit_conn.username, Message::EditConnUsername)
				]
				.spacing(5),
				row![
					text("Password")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("password", &app.edit_conn.password, Message::EditConnPassword)
				]
				.spacing(5),
				row![
					button(text("Submit")).on_press(Message::SubmitConnForm),
					button(text("Close")).on_press(Message::CloseConnForm),
				]
				.align_items(iced::Alignment::End)
				.spacing(5)
			]
			.spacing(10)
		]
		.spacing(20),
	)
	.padding(20)
	.width(Length::Fixed(400.0))
	.style(theme::Container::Box)
}
