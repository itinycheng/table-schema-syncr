use iced::{
	alignment::Vertical,
	theme,
	widget::{button, column, container, pick_list, row, text, text_input, Column, Container},
	Length, Renderer,
};

use super::{style::border_style, App, Message};
use crate::{database::DbType, gui::modal::Modal};

pub fn view<'a>(app: &App) -> Container<'a, Message, Renderer> {
	let mut content = container(Column::new().push(text("content")))
		.width(Length::FillPortion(4))
		.height(Length::Fill)
		.style(border_style());

	if app.show_conn_modal {
		content = container(
			Modal::new(content, connection_form(app)).on_blur(Message::CloseConnectionForm),
		)
		.width(Length::FillPortion(4))
		.height(Length::Fill)
		.style(border_style())
	}

	content
}

fn connection_form<'a>(app: &App) -> Container<'a, Message, Renderer> {
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
					button(text("Submit")).on_press(Message::SubmitConnectionForm),
					button(text("Close")).on_press(Message::CloseConnectionForm),
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
