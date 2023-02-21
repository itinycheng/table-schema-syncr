use iced::{
	alignment::Vertical,
	theme,
	widget::{button, column, container, row, text, text_input, Column, Container},
	Length, Renderer,
};

use super::{style::border_style, App, Message};
use crate::gui::modal::Modal;

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
			column![
				row![
					text("Type")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("type", &app.connection_params.db_type, Message::ConnectionDbType)
				]
				.spacing(5),
				row![
					text("Url")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("url", &app.connection_params.url, Message::ConnectionUrl)
				]
				.spacing(5),
				row![
					text("Username")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("username", &app.connection_params.username, Message::ConnectionUsername)
				]
				.spacing(5),
				row![
					text("Password")
						.size(16)
						.width(Length::Fixed(80.0))
						.vertical_alignment(Vertical::Center),
					text_input("password", &app.connection_params.password, Message::ConnectionPassword)
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
