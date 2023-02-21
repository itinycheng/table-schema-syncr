use iced::{
	widget::{container, text, Column, Container},
	Alignment, Length, Renderer,
};

use super::{style::border_style, Message};

pub fn view<'a>() -> Container<'a, Message, Renderer> {
	container(
		Column::new()
			.push(text("sidebar"))
			.max_width(150)
			.height(Length::Fill)
			.align_items(Alignment::Center),
	)
	.width(Length::FillPortion(1))
	.height(Length::Fill)
	.style(border_style())
}
