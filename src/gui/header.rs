use iced::{
	widget::{button, container, Row},
	Length, Renderer,
};

use super::{style::border_style, App, Message};

pub fn view<'a>(_: &App) -> iced::widget::Container<'a, Message, Renderer> {
	container(
		Row::new()
			.push(button("Create").on_press(Message::EditConnForm(None)))
			.push(button("blank").on_press(Message::Nothing))
			.spacing(10)
			.padding(10),
	)
	.width(Length::Fill)
	.style(border_style())
}
